use crate::formatters::Formatter;
use crate::rules::Severity;
use crate::scanner::ScanResult;

pub struct CompactFormatter;

impl Formatter for CompactFormatter {
    fn format(&self, results: &[ScanResult], quiet: bool) -> String {
        let mut output = String::new();

        for result in results {
            for v in &result.violations {
                if quiet && v.severity != Severity::Error {
                    continue;
                }

                let severity_str = match v.severity {
                    Severity::Error => "E",
                    Severity::Warning => "W",
                    Severity::Off => continue,
                };

                output.push_str(&format!(
                    "{sev}  {file}:{line}:{col}  {rule}  {msg}\n",
                    sev = severity_str,
                    file = v.file_path,
                    line = v.line,
                    col = v.column,
                    rule = v.rule_id,
                    msg = v.message,
                ));
            }
        }

        output
    }
}
