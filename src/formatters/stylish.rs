use std::fmt::Write;

use crate::formatters::Formatter;
use crate::rules::Severity;
use crate::scanner::ScanResult;

pub struct StylishFormatter;

impl Formatter for StylishFormatter {
    fn format(&self, results: &[ScanResult], quiet: bool) -> String {
        let mut output = String::new();
        let mut error_count = 0u32;
        let mut warning_count = 0u32;

        for result in results {
            let violations: Vec<_> = result
                .violations
                .iter()
                .filter(|v| {
                    if quiet {
                        v.severity == Severity::Error
                    } else {
                        true
                    }
                })
                .collect();

            if violations.is_empty() {
                continue;
            }

            let _ = writeln!(output, "\n{}:", result.file_path);

            for v in &violations {
                let severity_str = match v.severity {
                    Severity::Error => {
                        error_count += 1;
                        "error"
                    }
                    Severity::Warning => {
                        warning_count += 1;
                        "warning"
                    }
                    Severity::Off => continue,
                };

                let _ = writeln!(
                    output,
                    "  {severity_str}  {}:{}  {}  {}",
                    v.line, v.column, v.rule_id, v.message
                );
            }
        }

        let total = error_count + warning_count;
        if total > 0 {
            let _ = writeln!(output);
            if error_count > 0 {
                let _ = writeln!(output, "  {error_count} error(s)");
            }
            if warning_count > 0 {
                let _ = writeln!(output, "  {warning_count} warning(s)");
            }
        }

        output
    }
}
