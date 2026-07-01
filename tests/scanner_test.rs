#[cfg(test)]
mod tests {
    use react_auditor::rules::Severity;
    use react_auditor::scanner::Scanner;
    use std::collections::HashMap;

    #[test]
    fn test_severity_from_str() {
        assert_eq!("error".parse::<Severity>().unwrap(), Severity::Error);
        assert_eq!("ERROR".parse::<Severity>().unwrap(), Severity::Error);
        assert_eq!("warning".parse::<Severity>().unwrap(), Severity::Warning);
        assert_eq!("warn".parse::<Severity>().unwrap(), Severity::Warning);
        assert_eq!("off".parse::<Severity>().unwrap(), Severity::Off);
        assert_eq!("".parse::<Severity>().unwrap(), Severity::Off);
    }

    #[test]
    fn test_severity_is_on() {
        assert!(Severity::Error.is_on());
        assert!(Severity::Warning.is_on());
        assert!(!Severity::Off.is_on());
    }

    #[test]
    fn test_scanner_empty_files() {
        let scanner = Scanner::new(vec![], HashMap::new(), None, Vec::new());
        let results = scanner.scan().unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_no_console_rule_fires() {
        let code = "const x = 1;\nconsole.log('hello');\nconst y = 2;".to_string();
        let path = std::env::temp_dir().join("_test_no_console.js");
        std::fs::write(&path, &code).unwrap();

        let scanner = Scanner::new(
            vec![path.to_string_lossy().to_string()],
            HashMap::new(),
            None,
            Vec::new(),
        );
        let results = scanner.scan().unwrap();

        let has_violation = results
            .iter()
            .any(|r| r.violations.iter().any(|v| v.rule_id == "no-console"));
        assert!(has_violation, "Expected no-console violation to fire");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_no_console_rule_off_by_config() {
        let code = "console.log('hello');".to_string();
        let path = std::env::temp_dir().join("_test_no_console_off.js");
        std::fs::write(&path, &code).unwrap();

        let mut overrides = HashMap::new();
        overrides.insert("no-console".to_string(), "off".to_string());

        let scanner = Scanner::new(
            vec![path.to_string_lossy().to_string()],
            overrides,
            None,
            Vec::new(),
        );
        let results = scanner.scan().unwrap();

        let has_violation = results
            .iter()
            .any(|r| r.violations.iter().any(|v| v.rule_id == "no-console"));
        assert!(!has_violation, "Expected no-console to be disabled");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_scanner_parser_error_skips_file() {
        let code = "const x = ;".to_string();
        let path = std::env::temp_dir().join("_test_syntax_error.js");
        std::fs::write(&path, &code).unwrap();

        let scanner = Scanner::new(
            vec![path.to_string_lossy().to_string()],
            HashMap::new(),
            None,
            Vec::new(),
        );
        let results = scanner.scan().unwrap();
        assert!(
            results.is_empty(),
            "expected no results for syntax error file"
        );

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_scanner_non_code_file_skipped() {
        let code = "This is not code".to_string();
        let path = std::env::temp_dir().join("_test_non_code.txt");
        std::fs::write(&path, &code).unwrap();

        let scanner = Scanner::new(
            vec![path.to_string_lossy().to_string()],
            HashMap::new(),
            None,
            Vec::new(),
        );
        // .txt files won't match the default src/**/*.{js,jsx,ts,tsx} pattern
        // but if explicitly passed, the SourceType will fallback to default
        let _r = scanner.scan().unwrap();
        // should not panic on non-standard extension

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_scanner_directory_arg() {
        let scanner = Scanner::new(
            vec!["tests/fixtures".to_string()],
            HashMap::new(),
            None,
            Vec::new(),
        );
        let results = scanner.scan().unwrap();
        assert!(
            !results.is_empty(),
            "expected violations from fixtures directory"
        );
    }

    #[test]
    fn test_scanner_all_rule_ids_unique() {
        use react_auditor::rules::RuleRegistry;
        let registry = RuleRegistry::new();
        let ids = registry.get_rule_ids();
        let mut sorted = ids.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(ids.len(), sorted.len(), "rule IDs must be unique");
    }
}
