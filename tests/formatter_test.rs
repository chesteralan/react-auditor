use std::path::Path;

use react_auditor::formatters;
use react_auditor::rules::{Severity, Violation};
use react_auditor::scanner::ScanResult;

fn mock_results() -> Vec<ScanResult> {
    vec![
        ScanResult {
            file_path: "src/components/Button.tsx".to_string(),
            violations: vec![
                Violation {
                    file_path: "src/components/Button.tsx".to_string(),
                    line: 10,
                    column: 5,
                    rule_id: "no-console".to_string(),
                    category: "quality".to_string(),
                    message: "Unexpected console statement".to_string(),
                    severity: Severity::Error,
                },
                Violation {
                    file_path: "src/components/Button.tsx".to_string(),
                    line: 15,
                    column: 3,
                    rule_id: "no-var".to_string(),
                    category: "quality".to_string(),
                    message: "Unexpected var, use let or const instead".to_string(),
                    severity: Severity::Warning,
                },
            ],
        },
        ScanResult {
            file_path: "src/App.tsx".to_string(),
            violations: vec![Violation {
                file_path: "src/App.tsx".to_string(),
                line: 3,
                column: 7,
                rule_id: "no-missing-key".to_string(),
                category: "react".to_string(),
                message: "Missing key prop for list item".to_string(),
                severity: Severity::Error,
            }],
        },
    ]
}

fn read_snapshot(name: &str) -> String {
    let path = Path::new("tests").join("snapshots").join(name);
    std::fs::read_to_string(&path).unwrap_or_default()
}

fn write_snapshot(name: &str, content: &str) {
    let path = Path::new("tests").join("snapshots").join(name);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&path, content).expect("failed to write snapshot");
}

fn assert_snapshot(name: &str, actual: &str) {
    let expected = read_snapshot(name);
    if expected.is_empty() {
        // First run — create the snapshot
        write_snapshot(name, actual);
        eprintln!("Snapshot '{name}' created. Review and commit it.");
        return;
    }
    if expected != actual {
        let snap_path = Path::new("tests").join("snapshots").join(name);
        let update_path = snap_path.with_extension("new");
        std::fs::write(&update_path, actual).expect("failed to write updated snapshot");
        panic!(
            "Snapshot '{name}' does not match.\n  expected: {}\n  actual:   {}\n  diff saved to: {}",
            snap_path.display(),
            update_path.display(),
            update_path.display(),
        );
    }
}

#[test]
fn test_stylish_formatter_snapshot() {
    let results = mock_results();
    let formatter = formatters::get_formatter("stylish");
    let output = formatter.format(&results, false);
    assert_snapshot("stylish.txt", &output);
}

#[test]
fn test_json_formatter_snapshot() {
    let results = mock_results();
    let formatter = formatters::get_formatter("json");
    let output = formatter.format(&results, false);
    assert_snapshot("json.txt", &output);
}

#[test]
fn test_compact_formatter_snapshot() {
    let results = mock_results();
    let formatter = formatters::get_formatter("compact");
    let output = formatter.format(&results, false);
    assert_snapshot("compact.txt", &output);
}

#[test]
fn test_stylish_formatter_quiet_mode() {
    let results = mock_results();
    let formatter = formatters::get_formatter("stylish");
    let output = formatter.format(&results, true);
    // quiet mode should only show errors, not warnings
    assert!(output.contains("Unexpected console statement"));
    assert!(!output.contains("Unexpected var"));
}

#[test]
fn test_compact_formatter_quiet_mode() {
    let results = mock_results();
    let formatter = formatters::get_formatter("compact");
    let output = formatter.format(&results, true);
    assert!(output.contains("E  src/components/Button.tsx:10:5  no-console"));
    assert!(!output.contains("W  src/components/Button.tsx:15:3  no-var"));
}
