use std::path::Path;

use clap::Parser;

use react_auditor::cli::Cli;
use react_auditor::config::Config;
use react_auditor::formatters;
use react_auditor::rules::Severity;
use react_auditor::scanner::Scanner;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = Config::load(cli.config.as_ref().map(Path::new))?;

    let files = if cli.files.is_empty() {
        vec!["src/**/*.{js,jsx,ts,tsx}".to_string()]
    } else {
        cli.files.clone()
    };

    let scanner = Scanner::new(files, config.rules.clone());
    let results = scanner.scan()?;

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

    // Apply auto-fixes if requested
    if cli.fix && total_violations > 0 {
        let fix_count = scanner.apply_fixes(&results)?;
        if fix_count > 0 {
            eprintln!("Applied {fix_count} auto-fix(es). Re-scan to verify.");
            return Ok(());
        }
    }

    // Output via selected formatter
    let format_name = if cli.format == "stylish" && !config.format.is_empty() {
        &config.format
    } else {
        &cli.format
    };

    let formatter = formatters::get_formatter(format_name);
    let output = formatter.format(&results, cli.quiet);
    print!("{output}");

    // Write log file
    let log_path = cli.log.or_else(|| config.log.clone());
    if let Some(ref path) = log_path {
        let json_formatter = formatters::get_formatter("json");
        let log_output = json_formatter.format(&results, false);
        std::fs::write(path, log_output)?;
    }

    // Max warnings check
    let max_warnings = cli.max_warnings.or(config.max_warnings);
    let warnings_exceeded = max_warnings
        .map(|max| warning_count as u32 > max)
        .unwrap_or(false);

    if total_violations == 0 {
        println!("No issues found");
        Ok(())
    } else if warnings_exceeded {
        eprintln!(
            "Warning count ({warning_count}) exceeds max-warnings ({})",
            max_warnings.unwrap_or(0)
        );
        std::process::exit(1)
    } else if error_count > 0 {
        std::process::exit(1)
    } else {
        Ok(())
    }
}
