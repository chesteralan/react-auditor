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
    let scanner = Scanner::new(
        vec![path.to_string_lossy().to_string()],
        HashMap::new(),
        None,
        Vec::new(),
    );
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
    assert!(
        rule_ids.contains(&"no-unsafe-iframe".to_string()),
        "expected no-unsafe-iframe"
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
    assert!(
        rule_ids.contains(&"no-large-libraries".to_string()),
        "expected no-large-libraries"
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
    assert!(
        rule_ids.contains(&"a-has-content".to_string()),
        "expected a-has-content"
    );
    assert!(
        rule_ids.contains(&"no-ambiguous-labels".to_string()),
        "expected no-ambiguous-labels"
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

    let scanner = Scanner::new(
        vec![path.to_string_lossy().to_string()],
        HashMap::new(),
        None,
        Vec::new(),
    );
    let results = scanner.scan().unwrap();
    let total: usize = results.iter().map(|r| r.violations.len()).sum();
    assert_eq!(total, 0, "expected no violations for clean file");

    let _ = std::fs::remove_file(&path);
}

#[test]
fn e2e_react_issues_fires_rules() {
    let rule_ids = run_scanner("react_issues.jsx");
    assert!(
        rule_ids.contains(&"no-missing-key".to_string()),
        "expected no-missing-key"
    );
    assert!(
        rule_ids.contains(&"consistent-component-naming".to_string()),
        "expected consistent-component-naming"
    );
    assert!(
        rule_ids.contains(&"prefer-function-components".to_string()),
        "expected prefer-function-components"
    );
    assert!(
        rule_ids.contains(&"no-unnecessary-memo".to_string()),
        "expected no-unnecessary-memo"
    );
    assert!(
        rule_ids.contains(&"no-multiple-render-methods".to_string()),
        "expected no-multiple-render-methods"
    );
    assert!(
        rule_ids.contains(&"no-side-effects-in-render".to_string()),
        "expected no-side-effects-in-render"
    );
    assert!(
        rule_ids.contains(&"hook-rules".to_string()),
        "expected hook-rules"
    );
    assert!(
        rule_ids.contains(&"no-missing-deps".to_string()),
        "expected no-missing-deps"
    );
    assert!(
        rule_ids.contains(&"no-set-state-in-effect".to_string()),
        "expected no-set-state-in-effect"
    );
    assert!(
        rule_ids.contains(&"no-set-state-in-render".to_string()),
        "expected no-set-state-in-render"
    );
    assert!(
        rule_ids.contains(&"jsx-no-duplicate-props".to_string()),
        "expected jsx-no-duplicate-props"
    );
    assert!(!rule_ids.is_empty(), "expected at least one violation");
}

#[test]
fn e2e_quality_issues_fires_rules() {
    let rule_ids = run_scanner("quality_issues.jsx");
    assert!(
        rule_ids.contains(&"no-empty-blocks".to_string()),
        "expected no-empty-blocks"
    );
    assert!(
        rule_ids.contains(&"max-params".to_string()),
        "expected max-params"
    );
    assert!(
        rule_ids.contains(&"prefer-early-return".to_string()),
        "expected prefer-early-return"
    );
    assert!(
        rule_ids.contains(&"no-commented-code".to_string()),
        "expected no-commented-code"
    );
    assert!(
        rule_ids.contains(&"no-deep-nesting".to_string()),
        "expected no-deep-nesting"
    );
    assert!(
        rule_ids.contains(&"consistent-return".to_string()),
        "expected consistent-return"
    );
    assert!(
        rule_ids.contains(&"no-shadow".to_string()),
        "expected no-shadow"
    );
    assert!(
        rule_ids.contains(&"complexity".to_string()),
        "expected complexity"
    );
    assert!(!rule_ids.is_empty(), "expected at least one violation");
}

#[test]
fn e2e_nextjs_issues_fires_rules() {
    let rule_ids = run_scanner("nextjs_issues.jsx");
    assert!(
        rule_ids.contains(&"no-img-element".to_string()),
        "expected no-img-element"
    );
    assert!(
        rule_ids.contains(&"no-script-tag-in-head".to_string()),
        "expected no-script-tag-in-head"
    );
    assert!(
        rule_ids.contains(&"no-page-link".to_string()),
        "expected no-page-link"
    );
    assert!(
        rule_ids.contains(&"no-head-element".to_string()),
        "expected no-head-element"
    );
    assert!(
        rule_ids.contains(&"no-sync-script".to_string()),
        "expected no-sync-script"
    );
    assert!(!rule_ids.is_empty(), "expected at least one violation");
}

fn run_scanner_filtered(file: &str, categories: Vec<String>) -> Vec<String> {
    let path = fixture_path(file);
    let scanner = Scanner::new(
        vec![path.to_string_lossy().to_string()],
        HashMap::new(),
        Some(categories),
        Vec::new(),
    );
    let results = scanner.scan().unwrap();
    results
        .into_iter()
        .flat_map(|r| r.violations.into_iter().map(|v| v.rule_id))
        .collect()
}

#[test]
fn e2e_category_filter_limits_to_nextjs_only() {
    let rule_ids = run_scanner_filtered("nextjs_issues.jsx", vec!["nextjs".to_string()]);
    assert_eq!(
        rule_ids.len(),
        6,
        "expected 6 nextjs violations (2 no-page-link + no-sync-script)"
    );
    assert!(rule_ids.contains(&"no-img-element".to_string()));
    assert!(rule_ids.contains(&"no-script-tag-in-head".to_string()));
    assert!(rule_ids.contains(&"no-page-link".to_string()));
    assert!(rule_ids.contains(&"no-head-element".to_string()));
    assert!(rule_ids.contains(&"no-sync-script".to_string()));
}

#[test]
fn e2e_category_filter_empty_returns_nothing() {
    let rule_ids = run_scanner_filtered("has_issues.jsx", vec!["nonexistent".to_string()]);
    assert!(
        rule_ids.is_empty(),
        "expected no violations for nonexistent category"
    );
}

#[test]
fn e2e_new_rules_fires_rules() {
    let rule_ids = run_scanner("new_rules.tsx");
    assert!(
        rule_ids.contains(&"no-ref-in-component-name".to_string()),
        "expected no-ref-in-component-name"
    );
    assert!(
        rule_ids.contains(&"no-direct-mutation".to_string()),
        "expected no-direct-mutation"
    );
    assert!(
        rule_ids.contains(&"no-explicit-any".to_string()),
        "expected no-explicit-any"
    );
    assert!(!rule_ids.is_empty(), "expected at least one violation");
}

#[test]
fn e2e_no_console_fix_strips_console_calls() {
    let input = "function foo() {\n  console.log(\"test\");\n  return 1;\n}\n";
    let path = std::env::temp_dir().join("_e2e_no_console_fix.jsx");
    std::fs::write(&path, input).unwrap();
    let scanner = Scanner::new(
        vec![path.to_string_lossy().to_string()],
        HashMap::new(),
        None,
        Vec::new(),
    );
    let results = scanner.scan().unwrap();
    let mut fixed = input.to_string();
    for result in &results {
        for v in result.violations.iter().rev() {
            if let Some(rule) = scanner.registry.get_rule(&v.rule_id)
                && let Some(fix) = rule.fix(&v.to_finding(), &fixed)
                && fix.end <= fixed.len()
            {
                fixed.replace_range(fix.start..fix.end, &fix.replacement);
            }
        }
    }
    let expected = "function foo() {\n  \n  return 1;\n}\n";
    assert_eq!(fixed, expected, "console.log should be stripped");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn e2e_no_empty_blocks_fix_removes_empty_blocks() {
    let input = "function foo() {\n  if (true) {}\n}\n";
    let path = std::env::temp_dir().join("_e2e_no_empty_blocks_fix.jsx");
    std::fs::write(&path, input).unwrap();
    let scanner = Scanner::new(
        vec![path.to_string_lossy().to_string()],
        HashMap::new(),
        None,
        Vec::new(),
    );
    let results = scanner.scan().unwrap();
    let mut fixed = input.to_string();
    for result in &results {
        for v in result.violations.iter().rev() {
            if let Some(rule) = scanner.registry.get_rule(&v.rule_id)
                && let Some(fix) = rule.fix(&v.to_finding(), &fixed)
                && fix.end <= fixed.len()
            {
                fixed.replace_range(fix.start..fix.end, &fix.replacement);
            }
        }
    }
    let expected = "function foo() {\n  if (true) \n}\n";
    assert_eq!(fixed, expected, "empty block should be removed");
    let _ = std::fs::remove_file(&path);
}

fn version_binary() -> std::process::Command {
    let bin = std::env::var_os("CARGO_BIN_EXE_REACT_AUDITOR")
        .unwrap_or_else(|| {
            let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            p.push("target/debug/react-auditor");
            p.into_os_string()
        });
    std::process::Command::new(bin)
}

#[test]
fn e2e_version_flag() {
    let bin = version_binary()
        .arg("--version")
        .output()
        .expect("failed to run react-auditor --version");
    assert!(bin.status.success(), "--version should exit 0");
    let stdout = String::from_utf8_lossy(&bin.stdout);
    assert!(
        stdout.contains("react-auditor"),
        "output should contain binary name"
    );
    assert!(stdout.contains("0.1.7"), "output should contain version");
}

#[test]
fn e2e_short_version_flag() {
    let bin = version_binary()
        .arg("-V")
        .output()
        .expect("failed to run react-auditor -V");
    assert!(bin.status.success(), "-V should exit 0");
    let stdout = String::from_utf8_lossy(&bin.stdout);
    assert!(
        stdout.contains("react-auditor"),
        "output should contain binary name"
    );
    assert!(stdout.contains("0.1.7"), "output should contain version");
}
