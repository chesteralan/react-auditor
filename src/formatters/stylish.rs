use std::io::Write;

use termcolor::{Buffer, Color, ColorSpec, WriteColor};

use crate::formatters::Formatter;
use crate::rules::Severity;
use crate::scanner::ScanResult;

pub struct StylishFormatter;

impl Formatter for StylishFormatter {
    fn format(&self, results: &[ScanResult], quiet: bool) -> String {
        let mut buf = Buffer::ansi();
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

            let _ = writeln!(buf, "\n{}:", result.file_path);

            for v in &violations {
                let (severity_str, color) = match v.severity {
                    Severity::Error => {
                        error_count += 1;
                        ("error", Color::Red)
                    }
                    Severity::Warning => {
                        warning_count += 1;
                        ("warning", Color::Yellow)
                    }
                    Severity::Off => continue,
                };

                let _ = buf.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                let _ = write!(buf, "  {severity_str}  {}:{}  ", v.line, v.column);
                let _ = buf.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
                let _ = write!(buf, "[{}/{}]", v.category, v.rule_id);
                let _ = buf.set_color(ColorSpec::new().set_fg(Some(color)));
                let _ = write!(buf, "  {}", v.message);
                let _ = buf.reset();
                let _ = writeln!(buf);
            }
        }

        let total = error_count + warning_count;
        if total > 0 {
            let _ = writeln!(buf);
            if error_count > 0 {
                let _ = buf.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
                let _ = writeln!(buf, "  {error_count} error(s)");
            }
            if warning_count > 0 {
                let _ = buf.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true));
                let _ = writeln!(buf, "  {warning_count} warning(s)");
            }
            let _ = buf.reset();
        }

        String::from_utf8(buf.into_inner()).unwrap_or_default()
    }
}
