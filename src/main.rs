use std::path::Path;

use clap::Parser;
use globset::Glob;
use ignore::WalkBuilder;

use react_auditor::cli::{Cli, Commands};
use react_auditor::config::Config;
use react_auditor::formatters;
use react_auditor::presets::Preset;
use react_auditor::rules::Severity;
use react_auditor::scanner::Scanner;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            return react_auditor::init::install_pre_commit_hook();
        }
        None => {}
    }

    if cli.docs {
        react_auditor::docs::generate_docs();
        return Ok(());
    }

    let config = Config::load(cli.config.as_ref().map(Path::new))?;

    let preset: Preset = cli.preset.parse().unwrap_or(Preset::Recommended);

    let mut severity_overrides = preset.severity_overrides();
    severity_overrides.extend(config.rules.clone());

    let category_filter = if cli.rules.is_some() {
        cli.rules
            .as_ref()
            .map(|r| r.split(',').map(|s| s.trim().to_string()).collect())
    } else {
        preset.category_filter()
    };

    let files = if cli.files.is_empty() {
        let mut f: Vec<String> = vec!["src/**/*.{js,jsx,ts,tsx}".to_string()];

        // Expand workspace globs from config and use them as scan roots
        if !config.workspaces.is_empty() {
            f = expand_workspace_roots(&config.workspaces);
        }

        f
    } else {
        cli.files.clone()
    };

    let ignore_patterns: Vec<String> = if cli.ignore.is_empty() {
        Vec::new()
    } else {
        cli.ignore
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    let mut scanner = Scanner::new(files, severity_overrides, category_filter, ignore_patterns);
    scanner.use_cache = !cli.no_cache;
    scanner.file_type_overrides = config.file_types.clone();

    if cli.watch {
        return react_auditor::watch::watch(&scanner);
    }

    let results = scanner.scan()?;
    print_results(&cli, &config, &scanner, &results)
}

/// Expand workspace glob patterns to matching directory roots.
fn expand_workspace_roots(patterns: &[String]) -> Vec<String> {
    let mut roots = Vec::new();
    for pattern in patterns {
        if Path::new(pattern).is_dir() {
            roots.push(pattern.clone());
            continue;
        }
        let glob = match Glob::new(pattern) {
            Ok(g) => g.compile_matcher(),
            Err(_) => continue,
        };
        for entry in WalkBuilder::new(".").max_depth(Some(3)).build().flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) && glob.is_match(entry.path())
            {
                roots.push(entry.path().to_string_lossy().to_string());
            }
        }
    }
    roots.sort();
    roots.dedup();
    roots
}

fn print_results(
    cli: &Cli,
    config: &Config,
    scanner: &Scanner,
    results: &[react_auditor::scanner::ScanResult],
) -> anyhow::Result<()> {
    let total_violations: usize = results.iter().map(|r| r.violations.len()).sum();
    let error_count: usize = results
        .iter()
        .flat_map(|r| &r.violations)
        .filter(|v| v.severity == Severity::Error)
        .count();
    let warning_count: usize = results
        .iter()
        .flat_map(|r| &r.violations)
        .filter(|v| v.severity == Severity::Warning)
        .count();

    if cli.fix && total_violations > 0 {
        let fix_count = scanner.apply_fixes(results)?;
        if fix_count > 0 {
            eprintln!("Applied {fix_count} auto-fix(es). Re-scan to verify.");
            return Ok(());
        }
    }

    let format_name = if cli.format == "stylish" && !config.format.is_empty() {
        &config.format
    } else {
        &cli.format
    };

    let formatter = formatters::get_formatter(format_name);
    let output = formatter.format(results, cli.quiet);
    print!("{output}");

    let log_path = cli.log.clone().or_else(|| config.log.clone());
    if let Some(ref path) = log_path {
        let json_formatter = formatters::get_formatter("json");
        let log_output = json_formatter.format(results, false);
        std::fs::write(path, log_output)?;
    }

    let max_warnings = cli.max_warnings.or(config.max_warnings);
    let warnings_exceeded = max_warnings
        .map(|max| warning_count as u32 > max)
        .unwrap_or(false);

    let fail_on_error = cli.fail_on == "error" && error_count > 0;
    let fail_on_warning = cli.fail_on == "warning" && total_violations > 0;

    if total_violations == 0 {
        println!("No issues found");
        Ok(())
    } else if warnings_exceeded {
        eprintln!(
            "Warning count ({warning_count}) exceeds max-warnings ({})",
            max_warnings.unwrap_or(0)
        );
        std::process::exit(1)
    } else if fail_on_error || fail_on_warning {
        std::process::exit(1)
    } else {
        Ok(())
    }
}
