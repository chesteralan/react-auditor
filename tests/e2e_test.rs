use std::collections::HashMap;
use std::path::PathBuf;

use react_auditor::scanner::Scanner;

fn fixture_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures");
    p.push(name);
    p
}

fn run_scanner(file: &str) -> Vec<String> {
    let path = fixture_path(file);
    let scanner = Scanner::new(vec![path.to_string_lossy().to_string()], HashMap::new());
    let results = scanner.scan().unwrap();
    results
        .into_iter()
        .flat_map(|r| r.violations.into_iter().map(|v| v.rule_id))
        .collect()
}

#[test]
fn e2e_has_issues_fires_rules() {
    let rule_ids = run_scanner("has_issues.jsx");
    assert!(rule_ids.contains(&"no-var".to_string()), "expected no-var");
    assert!(
        rule_ids.contains(&"no-console".to_string()),
        "expected no-console"
    );
    assert!(
        rule_ids.contains(&"no-inline-styles".to_string()),
        "expected no-inline-styles"
    );
    assert!(
        rule_ids.contains(&"no-index-key".to_string()),
        "expected no-index-key"
    );
    assert!(
        rule_ids.contains(&"no-inline-functions".to_string()),
        "expected no-inline-functions"
    );
    assert!(!rule_ids.is_empty(), "expected at least one violation");
}

#[test]
fn e2e_typescript_issues_fires_rules() {
    let rule_ids = run_scanner("typescript_issues.ts");
    assert!(rule_ids.contains(&"no-any".to_string()), "expected no-any");
    assert!(
        rule_ids.contains(&"no-non-null-assertion".to_string()),
        "expected no-non-null-assertion"
    );
    assert!(
        rule_ids.contains(&"no-type-assertion".to_string()),
        "expected no-type-assertion"
    );
    assert!(
        rule_ids.contains(&"no-empty-interface".to_string()),
        "expected no-empty-interface"
    );
    assert!(
        rule_ids.contains(&"prefer-interface".to_string()),
        "expected prefer-interface"
    );
    assert!(!rule_ids.is_empty(), "expected at least one violation");
}

#[test]
fn e2e_security_issues_fires_rules() {
    let rule_ids = run_scanner("security_issues.jsx");
    assert!(
        rule_ids.contains(&"no-dangerously-set-innerhtml".to_string()),
        "expected no-dangerously-set-innerhtml"
    );
    assert!(
        rule_ids.contains(&"no-insecure-protocol".to_string()),
        "expected no-insecure-protocol"
    );
    assert!(!rule_ids.is_empty(), "expected at least one violation");
}

#[test]
fn e2e_performance_issues_fires_rules() {
    let rule_ids = run_scanner("performance_issues.jsx");
    assert!(
        rule_ids.contains(&"no-inline-functions".to_string()),
        "expected no-inline-functions"
    );
    assert!(
        rule_ids.contains(&"prefer-fragments".to_string()),
        "expected prefer-fragments"
    );
    assert!(
        rule_ids.contains(&"no-heavy-computation-in-render".to_string()),
        "expected no-heavy-computation-in-render"
    );
    assert!(!rule_ids.is_empty(), "expected at least one violation");
}

#[test]
fn e2e_accessibility_issues_fires_rules() {
    let rule_ids = run_scanner("accessibility_issues.jsx");
    assert!(
        rule_ids.contains(&"img-alt".to_string()),
        "expected img-alt"
    );
    assert!(
        rule_ids.contains(&"button-has-type".to_string()),
        "expected button-has-type"
    );
    assert!(
        rule_ids.contains(&"label-associated".to_string()),
        "expected label-associated"
    );
    assert!(
        rule_ids.contains(&"heading-levels".to_string()),
        "expected heading-levels"
    );
    assert!(!rule_ids.is_empty(), "expected at least one violation");
}

#[test]
fn e2e_no_false_positives_for_clean_fixtures() {
    let clean = r#"
function Greeting(): JSX.Element {
  return <h1>Hello, World!</h1>;
}

export default Greeting;
"#;
    let path = std::env::temp_dir().join("_e2e_clean.tsx");
    std::fs::write(&path, clean).unwrap();

    let scanner = Scanner::new(vec![path.to_string_lossy().to_string()], HashMap::new());
    let results = scanner.scan().unwrap();
    let total: usize = results.iter().map(|r| r.violations.len()).sum();
    assert_eq!(total, 0, "expected no violations for clean file");

    let _ = std::fs::remove_file(&path);
}
