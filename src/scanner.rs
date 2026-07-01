use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use ignore::WalkBuilder;
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;

use crate::rules::{RuleRegistry, Violation};

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub file_path: String,
    pub violations: Vec<Violation>,
}

pub struct Scanner {
    pub files: Vec<String>,
    pub registry: RuleRegistry,
    pub severity_overrides: HashMap<String, String>,
}

impl Scanner {
    pub fn new(files: Vec<String>, severity_overrides: HashMap<String, String>) -> Self {
        Self {
            files,
            registry: RuleRegistry::new(),
            severity_overrides,
        }
    }

    pub fn scan(&self) -> Result<Vec<ScanResult>> {
        let paths = self.resolve_files()?;
        let mut results = Vec::new();

        for path_str in &paths {
            let path = Path::new(path_str);
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("Failed to read {path_str}"))?;

            let source_type = SourceType::from_path(path).unwrap_or_default();

            let allocator = Allocator::default();
            let ret = Parser::new(&allocator, &content, source_type).parse();

            if !ret.errors.is_empty() {
                continue;
            }

            let program = allocator.alloc(ret.program);
            let semantic = SemanticBuilder::new().build(program);

            let violations = self.registry.run_rules(
                program,
                &semantic.semantic,
                &content,
                path_str,
                &self.severity_overrides,
            );

            if !violations.is_empty() {
                results.push(ScanResult {
                    file_path: path_str.clone(),
                    violations,
                });
            }
        }

        Ok(results)
    }

    fn resolve_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();

        if self.files.is_empty() {
            for entry in WalkBuilder::new("src").standard_filters(true).build() {
                if let Ok(entry) = entry
                    && entry.file_type().map(|t| t.is_file()).unwrap_or(false)
                    && let Some(ext) = entry.path().extension().and_then(|e| e.to_str())
                    && matches!(ext, "js" | "jsx" | "ts" | "tsx")
                {
                    files.push(entry.path().to_string_lossy().to_string());
                }
            }
        } else {
            for pattern in &self.files {
                let path = Path::new(pattern);
                if path.is_file() {
                    files.push(pattern.clone());
                } else {
                    let glob_pattern = globset::Glob::new(pattern)
                        .with_context(|| format!("Invalid glob pattern: {pattern}"))?
                        .compile_matcher();

                    for entry in WalkBuilder::new(".").standard_filters(true).build() {
                        if let Ok(entry) = entry
                            && entry.file_type().map(|t| t.is_file()).unwrap_or(false)
                            && glob_pattern.is_match(entry.path())
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
                let Some(replacement) = rule.fix(&v.to_finding(), &fixed) else {
                    continue;
                };

                let offset = match line_col_to_offset(&fixed, v.line, v.column) {
                    Some(o) => o,
                    None => continue,
                };

                let var_len = fixed[offset..].find([' ', '\t', '\n', ';']).unwrap_or(3);
                fixed.replace_range(offset..offset + var_len, &replacement);
                total += 1;
            }

            if total > 0 {
                std::fs::write(path, &fixed)
                    .with_context(|| format!("Failed to write {}", result.file_path))?;
            }
        }

        Ok(total)
    }
}

fn line_col_to_offset(source: &str, line: usize, col: usize) -> Option<usize> {
    let mut current_line = 1;
    let mut offset = 0;
    for (i, _) in source.char_indices() {
        if current_line == line {
            return Some(offset + col - 1);
        }
        if source.as_bytes().get(i) == Some(&b'\n') {
            current_line += 1;
            offset = i + 1;
        }
    }
    None
}
