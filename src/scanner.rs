use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use globset::GlobBuilder;
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_semantic::{Semantic, SemanticBuilder};
use oxc_span::SourceType;
use rayon::prelude::*;

use crate::cache::Cache;
use crate::rules::{RuleRegistry, Severity, Violation};

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub file_path: String,
    pub violations: Vec<Violation>,
}

pub struct Scanner {
    pub files: Vec<String>,
    pub registry: RuleRegistry,
    pub severity_overrides: HashMap<String, String>,
    pub category_filter: Option<Vec<String>>,
    pub use_cache: bool,
    pub file_type_overrides: HashMap<String, HashMap<String, String>>,
    ignore_patterns: Vec<String>,
}

impl Scanner {
    pub fn new(
        files: Vec<String>,
        severity_overrides: HashMap<String, String>,
        category_filter: Option<Vec<String>>,
        ignore_patterns: Vec<String>,
    ) -> Self {
        Self {
            files,
            registry: RuleRegistry::new(),
            severity_overrides,
            category_filter,
            use_cache: true,
            file_type_overrides: HashMap::new(),
            ignore_patterns,
        }
    }

    fn semantic_is_needed(&self) -> bool {
        let needed_ids = ["no-shadow", "no-unused-vars"];
        needed_ids.iter().any(|id| {
            self.severity_overrides
                .get(*id)
                .map(|s| s.parse::<Severity>().unwrap_or(Severity::Off).is_on())
                .unwrap_or_else(|| {
                    self.registry
                        .get_rule(id)
                        .map(|r| r.meta().default_severity.is_on())
                        .unwrap_or(false)
                })
        })
    }

    fn is_ignored(&self, path: &Path) -> bool {
        if self.ignore_patterns.is_empty() {
            return false;
        }
        let path_str = path.to_string_lossy();
        self.ignore_patterns.iter().any(|pattern| {
            if let Ok(glob) = GlobBuilder::new(pattern).literal_separator(true).build() {
                let matcher = glob.compile_matcher();
                matcher.is_match(path_str.as_ref())
            } else {
                false
            }
        })
    }

    fn walk_files(&self, root: &Path) -> Vec<String> {
        let mut files = Vec::new();
        for result in WalkBuilder::new(root).standard_filters(true).require_git(false).build() {
            if let Ok(entry) = result
                && entry.file_type().map(|t| t.is_file()).unwrap_or(false)
                && let Some(ext) = entry.path().extension().and_then(|e| e.to_str())
                && matches!(ext, "js" | "jsx" | "ts" | "tsx")
                && !self.is_ignored(entry.path())
            {
                files.push(entry.path().to_string_lossy().to_string());
            }
        }
        files
    }

    fn compute_merged_overrides_map(&self) -> HashMap<String, HashMap<String, String>> {
        let mut map = HashMap::new();
        for ext in self.file_type_overrides.keys() {
            let mut merged = self.severity_overrides.clone();
            if let Some(overrides) = self.file_type_overrides.get(ext.as_str()) {
                for (rule_id, severity) in overrides {
                    merged.insert(rule_id.clone(), severity.clone());
                }
            }
            map.insert(ext.clone(), merged);
        }
        map
    }

    pub fn scan(&self) -> Result<Vec<ScanResult>> {
        let mut cache = Cache::load();
        let all_paths = self.resolve_files()?;

        // Pre-compute merged overrides per extension
        let merged_map = self.compute_merged_overrides_map();

        // Filter to only scan files that changed or weren't cached as clean
        let paths: Vec<String> = if self.use_cache {
            all_paths
                .into_iter()
                .filter(|p| !cache.is_unchanged_clean(Path::new(p)))
                .collect()
        } else {
            all_paths
        };

        let total = paths.len();

        let pb = if total > 1 {
            let bar = ProgressBar::new(total as u64);
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:32.cyan/blue}] {pos}/{len}  {msg}")
                    .unwrap()
                    .progress_chars("=> "),
            );
            bar.set_message("scanning...");
            Some(bar)
        } else {
            None
        };

        let results: Vec<ScanResult> = paths
            .par_iter()
            .filter_map(|path_str| {
                if let Some(ref bar) = pb {
                    bar.set_message(path_str.to_string());
                }

                let path = Path::new(path_str);
                let content = match std::fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(_) => {
                        if let Some(ref bar) = pb {
                            bar.inc(1);
                        }
                        return None;
                    }
                };

                let needs_semantic = self.semantic_is_needed();
                let source_type = SourceType::from_path(path).unwrap_or_default();
                let allocator = Allocator::default();
                let ret = Parser::new(&allocator, &content, source_type).parse();

                if !ret.diagnostics.is_empty() {
                    if let Some(ref bar) = pb {
                        bar.inc(1);
                    }
                    return None;
                }

                let program = allocator.alloc(ret.program);
                let semantic = if needs_semantic {
                    SemanticBuilder::new().build(program).semantic
                } else {
                    Semantic::default()
                };

                let overrides = if merged_map.is_empty() {
                    &self.severity_overrides
                } else {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    merged_map.get(ext).unwrap_or(&self.severity_overrides)
                };

                let violations = self.registry.run_rules(
                    program,
                    &semantic,
                    &content,
                    path_str,
                    overrides,
                    self.category_filter.as_ref(),
                );

                if let Some(ref bar) = pb {
                    bar.inc(1);
                }

                if violations.is_empty() {
                    None
                } else {
                    Some(ScanResult {
                        file_path: path_str.clone(),
                        violations,
                    })
                }
            })
            .collect();

        // Update cache for scanned files
        for result in &results {
            cache.mark_dirty(Path::new(&result.file_path));
        }
        for path_str in &paths {
            if !results.iter().any(|r| r.file_path == *path_str) {
                cache.mark_clean(Path::new(path_str));
            }
        }
        cache.save();

        if let Some(bar) = pb {
            let v = results.iter().map(|r| r.violations.len()).sum::<usize>();
            bar.finish_with_message(format!("{v} violation(s) in {} file(s)", results.len()));
        }

        Ok(results)
    }

    /// Scan a specific list of file paths without resolving globs / applying ignores.
    pub fn scan_paths(&self, paths: &[String]) -> Result<Vec<ScanResult>> {
        let total = paths.len();
        let merged_map = self.compute_merged_overrides_map();

        let pb = if total > 1 {
            let bar = ProgressBar::new(total as u64);
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:32.cyan/blue}] {pos}/{len}  {msg}")
                    .unwrap()
                    .progress_chars("=> "),
            );
            bar.set_message("scanning...");
            Some(bar)
        } else {
            None
        };

        let results: Vec<ScanResult> = paths
            .par_iter()
            .filter_map(|path_str| {
                if let Some(ref bar) = pb {
                    bar.set_message(path_str.to_string());
                }

                let path = Path::new(path_str);
                let content = std::fs::read_to_string(path).ok()?;
                let source_type = SourceType::from_path(path).unwrap_or_default();
                let allocator = Allocator::default();
                let ret = Parser::new(&allocator, &content, source_type).parse();

                if !ret.diagnostics.is_empty() {
                    if let Some(ref bar) = pb {
                        bar.inc(1);
                    }
                    return None;
                }

                let needs_semantic = self.semantic_is_needed();
                let program = allocator.alloc(ret.program);
                let semantic = if needs_semantic {
                    SemanticBuilder::new().build(program).semantic
                } else {
                    Semantic::default()
                };

                let overrides = if merged_map.is_empty() {
                    &self.severity_overrides
                } else {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    merged_map.get(ext).unwrap_or(&self.severity_overrides)
                };

                let violations = self.registry.run_rules(
                    program,
                    &semantic,
                    &content,
                    path_str,
                    overrides,
                    self.category_filter.as_ref(),
                );

                if let Some(ref bar) = pb {
                    bar.inc(1);
                }

                if violations.is_empty() {
                    None
                } else {
                    Some(ScanResult {
                        file_path: path_str.clone(),
                        violations,
                    })
                }
            })
            .collect();

        if let Some(bar) = pb {
            let v = results.iter().map(|r| r.violations.len()).sum::<usize>();
            bar.finish_with_message(format!("{v} violation(s) in {} file(s)", results.len()));
        }

        Ok(results)
    }

    fn resolve_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();

        if self.files.is_empty() {
            files = self.walk_files(Path::new("src"));
        } else {
            for pattern in &self.files {
                let path = Path::new(pattern);
                if path.is_file() {
                    files.push(pattern.clone());
                } else if path.is_dir() {
                    files.extend(self.walk_files(path));
                } else {
                    let glob_pattern = globset::Glob::new(pattern)
                        .with_context(|| format!("Invalid glob pattern: {pattern}"))?
                        .compile_matcher();

                    for entry in WalkBuilder::new(".").standard_filters(true).require_git(false).build() {
                        if let Ok(entry) = entry
                            && entry.file_type().map(|t| t.is_file()).unwrap_or(false)
                            && glob_pattern.is_match(entry.path())
                            && !self.is_ignored(entry.path())
                        {
                            files.push(entry.path().to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    pub fn apply_fixes(&self, results: &[ScanResult]) -> Result<usize> {
        let mut total = 0;

        for result in results {
            let path = Path::new(&result.file_path);
            let source = std::fs::read_to_string(path)
                .with_context(|| format!("Failed to read {}", result.file_path))?;
            let mut fixed = source.clone();

            for v in result.violations.iter().rev() {
                let Some(rule) = self.registry.get_rule(&v.rule_id) else {
                    continue;
                };
                let Some(fix) = rule.fix(&v.to_finding(), &fixed) else {
                    continue;
                };

                if fix.end <= fixed.len() {
                    fixed.replace_range(fix.start..fix.end, &fix.replacement);
                    total += 1;
                }
            }

            if total > 0 {
                std::fs::write(path, &fixed)
                    .with_context(|| format!("Failed to write {}", result.file_path))?;
            }
        }

        Ok(total)
    }
}
