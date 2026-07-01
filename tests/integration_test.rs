use std::collections::HashMap;

use react_auditor::scanner::Scanner;

const PROJECT_DIR: &str = "tests/real-project/src";

#[test]
fn integration_scan_real_project_produces_violations() {
    let scanner = Scanner::new(
        vec![PROJECT_DIR.to_string()],
        HashMap::new(),
        None,
        Vec::new(),
    );
    let results = scanner.scan().unwrap();
    assert!(!results.is_empty(), "Expected violations in real-project");
}

#[test]
fn integration_scan_real_project_no_panic() {
    let scanner = Scanner::new(
        vec![PROJECT_DIR.to_string()],
        HashMap::new(),
        None,
        Vec::new(),
    );
    let results = scanner.scan().unwrap();
    // Should complete without panic; collect results into a string
    let output = format!("{:?}", results);
    assert!(!output.is_empty());
}

#[test]
fn integration_scan_real_project_with_category_filter() {
    let scanner = Scanner::new(
        vec![PROJECT_DIR.to_string()],
        HashMap::new(),
        Some(vec!["typescript".to_string()]),
        Vec::new(),
    );
    let results = scanner.scan().unwrap();
    for result in &results {
        for v in &result.violations {
            assert_eq!(v.category, "typescript", "Expected only TS violations");
        }
    }
}

#[test]
fn integration_scan_real_project_with_ignore() {
    let scanner = Scanner::new(
        vec![PROJECT_DIR.to_string()],
        HashMap::new(),
        None,
        vec!["**/helpers.ts".to_string()],
    );
    let results = scanner.scan().unwrap();
    let has_helpers = results.iter().any(|r| r.file_path.contains("helpers.ts"));
    assert!(!has_helpers, "helpers.ts should be ignored");
}

#[test]
fn integration_scan_real_project_json_output() {
    let scanner = Scanner::new(
        vec![PROJECT_DIR.to_string()],
        HashMap::new(),
        None,
        Vec::new(),
    );
    let results = scanner.scan().unwrap();
    let formatter = react_auditor::formatters::get_formatter("json");
    let output = formatter.format(&results, false);
    // Verify it's valid JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&output).expect("JSON formatter output should be valid JSON");
    assert!(parsed.is_array(), "JSON output should be an array");
    assert!(
        !parsed.as_array().unwrap().is_empty(),
        "JSON should have entries"
    );
}

#[test]
fn integration_scan_real_project_compact_output() {
    let scanner = Scanner::new(
        vec![PROJECT_DIR.to_string()],
        HashMap::new(),
        None,
        Vec::new(),
    );
    let results = scanner.scan().unwrap();
    let formatter = react_auditor::formatters::get_formatter("compact");
    let output = formatter.format(&results, false);
    // Compact output has lines like "E  file:line:col  rule  msg"
    for line in output.lines() {
        assert!(
            line.starts_with("E ") || line.starts_with("W "),
            "Each line should start with E or W: {line}"
        );
    }
}

#[test]
fn integration_scan_real_project_stylish_output() {
    let scanner = Scanner::new(
        vec![PROJECT_DIR.to_string()],
        HashMap::new(),
        None,
        Vec::new(),
    );
    let results = scanner.scan().unwrap();
    let formatter = react_auditor::formatters::get_formatter("stylish");
    let output = formatter.format(&results, false);
    // Stylish output should contain file paths
    for result in &results {
        assert!(
            output.contains(&result.file_path),
            "Stylish output should contain file path: {}",
            result.file_path
        );
    }
}

#[test]
fn integration_scan_with_severity_overrides() {
    let mut overrides = HashMap::new();
    overrides.insert("no-console".to_string(), "off".to_string());
    overrides.insert("no-var".to_string(), "off".to_string());

    let scanner = Scanner::new(vec![PROJECT_DIR.to_string()], overrides, None, Vec::new());
    let results = scanner.scan().unwrap();

    let has_console = results
        .iter()
        .flat_map(|r| &r.violations)
        .any(|v| v.rule_id == "no-console");
    let has_var = results
        .iter()
        .flat_map(|r| &r.violations)
        .any(|v| v.rule_id == "no-var");
    assert!(!has_console, "no-console should be disabled");
    assert!(!has_var, "no-var should be disabled");
}
