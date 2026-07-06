use crate::formatters::Formatter;
use crate::rules::Severity;
use crate::scanner::ScanResult;

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, results: &[ScanResult], _quiet: bool) -> String {
        let entries: Vec<serde_json::Value> = results
            .iter()
            .flat_map(|r| {
                r.violations.iter().map(|v| {
                    serde_json::json!({
                        "file": v.file_path,
                        "line": v.line,
                        "column": v.column,
                        "ruleId": v.rule_id,
                        "category": v.category,
                        "message": v.message,
                        "severity": match v.severity {
                            Severity::Error => "error",
                            Severity::Warning => "warning",
                            Severity::Off => "off",
                        },
                    })
                })
            })
            .collect();

        serde_json::to_string_pretty(&entries).unwrap_or_default()
    }
}
