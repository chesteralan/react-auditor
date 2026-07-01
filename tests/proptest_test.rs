use std::collections::HashMap;

use proptest::prelude::*;

use react_auditor::rules::RuleRegistry;
use react_auditor::scanner::Scanner;

/// Templates of valid JS snippets used to compose random programs.
const JS_STMTS: &[&str] = &[
    "const x = 1;",
    "let y = 'hello';",
    "var z = true;",
    "if (true) {}",
    "if (false) { const a = 1; }",
    "for (let i = 0; i < 10; i++) {}",
    "while (false) {}",
    "function foo() { return 1; }",
    "const bar = () => {};",
    "const obj = { a: 1, b: 2 };",
    "const arr = [1, 2, 3];",
    "console.log('test');",
    "const el = <div />;",
    "const el2 = <div className=\"x\">hello</div>;",
    "export const X = () => <span />;",
    "import React from 'react';",
    "import { useState } from 'react';",
    "const [a, setA] = useState(0);",
    "a + b;",
    "x?.y?.z;",
    "const { a, b } = obj;",
    "const [head, ...rest] = arr;",
    "try { foo() } catch (e) {}",
    "switch (x) { case 1: break; default: break; }",
    "class MyComp extends React.Component { render() { return null; } }",
];

fn scan_source(source: &str, ext: &str) {
    let dir = std::env::temp_dir().join("_proptest_scan");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(format!("test.{}", ext));
    let _ = std::fs::write(&path, source);

    let scanner = Scanner::new(
        vec![path.to_string_lossy().to_string()],
        HashMap::new(),
        None,
        Vec::new(),
    );
    let _ = scanner.scan();
    let _ = std::fs::remove_file(&path);
}

proptest! {
    #[test]
    fn no_panic_on_random_js(stmts in prop::collection::vec(0..JS_STMTS.len() as i32, 0..10)) {
        let source = stmts.iter()
            .map(|&i| JS_STMTS[i as usize])
            .collect::<Vec<_>>()
            .join("\n");
        scan_source(&source, "jsx");
    }

    #[test]
    fn no_panic_on_many_imports(count in 0usize..50) {
        let mut source = String::from("import React from 'react';\n");
        for i in 0..count {
            source.push_str(&format!("import {{ foo_{i} }} from './mod_{i}';\n"));
        }
        source.push_str("const x = 1;\n");
        scan_source(&source, "jsx");
    }

    #[test]
    fn no_panic_on_deeply_nested_blocks(depth in 0usize..20) {
        let mut source = String::new();
        for _ in 0..depth {
            source.push_str("if (true) { ");
        }
        source.push_str("const x = 1;");
        for _ in 0..depth {
            source.push('}');
        }
        scan_source(&source, "jsx");
    }
}

#[test]
fn no_panic_on_empty_string() {
    scan_source("", "jsx");
}

#[test]
fn no_panic_on_unicode_source() {
    scan_source("// 中文\nconst x = '你好';\nconst y = '👋';\n", "jsx");
}

#[test]
fn no_panic_on_very_large_number_literal() {
    scan_source("const x = 1e999;", "jsx");
}

#[test]
fn no_panic_on_template_literals() {
    scan_source("const x = `hello ${name} world`;", "jsx");
}

#[test]
fn no_panic_on_tagged_templates() {
    scan_source("const x = html`<div>hello</div>`;", "jsx");
}

#[test]
fn no_panic_on_hooks() {
    scan_source(
        r#"
import { useState, useEffect, useCallback, useMemo } from 'react';
function MyComp() {
  const [count, setCount] = useState(0);
  useEffect(() => {}, []);
  const cb = useCallback(() => {}, []);
  const val = useMemo(() => count * 2, [count]);
  return <div>{count}</div>;
}
"#,
        "jsx",
    );
}

#[test]
fn no_panic_on_classes() {
    scan_source(
        r#"
class A extends B {
  constructor() { super(); }
  method() { return this.x; }
}
const instance = new A();
"#,
        "jsx",
    );
}

#[test]
fn no_panic_on_jsx_edge_cases() {
    scan_source(
        r#"
const a = <></>;
const b = <div {...spread} />;
const c = <div ref={r} key="k" />;
const d = <div dangerouslySetInnerHTML={{__html: '<p>x</p>'}} />;
const e = <img src="x" />;
const f = <label><input /></label>;
"#,
        "jsx",
    );
}

#[test]
fn no_panic_on_type_annotations() {
    scan_source(
        r#"
const a: string = 'hello';
let b: number | undefined;
function fn(x: string, y?: number): void {}
const cb: (a: string) => void = (x) => {};
interface Props { name: string; age?: number; }
type MyType = string | number;
"#,
        "tsx",
    );
}

#[test]
fn no_panic_on_async_and_generators() {
    scan_source(
        r#"
async function fetchData(url: string): Promise<any> {
  const res = await fetch(url);
  return res.json();
}
function* gen() { yield 1; yield 2; }
async function* asyncGen() { yield await Promise.resolve(1); }
"#,
        "tsx",
    );
}

#[test]
fn all_rules_have_valid_metadata() {
    let registry = RuleRegistry::new();
    let ids = registry.get_rule_ids();
    for id in &ids {
        let rule = registry.get_rule(id);
        assert!(rule.is_some(), "Rule '{id}' registered but not findable");
        let meta = rule.unwrap().meta();
        assert!(!meta.id.is_empty(), "Rule has empty id");
        assert!(!meta.category.is_empty(), "Rule '{id}' has empty category");
        assert!(
            !meta.description.is_empty(),
            "Rule '{id}' has empty description"
        );
    }
}

// ── Fuzz-style tests: random byte sequences ──

#[test]
fn fuzz_empty_bytes() {
    scan_source("", "jsx");
    scan_source("", "tsx");
}

#[test]
fn fuzz_only_whitespace() {
    scan_source("   \n  \t  \n  ", "jsx");
}

#[test]
fn fuzz_only_special_chars() {
    scan_source("!@#$%^&*()_+-=[]{}|;':\",./<>?`~", "jsx");
}

#[test]
fn fuzz_repeated_opening_braces() {
    let input = "{".repeat(1000);
    scan_source(&input, "jsx");
}

#[test]
fn fuzz_repeated_closing_braces() {
    let input = "}".repeat(1000);
    scan_source(&input, "jsx");
}

#[test]
fn fuzz_malformed_json_in_jsx() {
    scan_source("const x = <div data={{broken json}} />;", "jsx");
}

#[test]
fn fuzz_extremely_long_identifier() {
    let id = "a".repeat(10000);
    let source = format!("const {id} = 1;");
    scan_source(&source, "jsx");
}

#[test]
fn fuzz_backtick_nesting() {
    scan_source("const x = `outer ${`inner ${`deep`}`}`;", "jsx");
}

#[test]
fn fuzz_unicode_control_chars() {
    let source: String = (0u8..32).map(|c| c as char).collect();
    scan_source(&source, "jsx");
}

#[test]
fn fuzz_null_bytes() {
    let source = "const x = \0 'hello';";
    scan_source(source, "jsx");
}

#[test]
fn fuzz_zero_width_chars() {
    scan_source("const x = '\u{200B}\u{200C}\u{200D}';", "jsx");
}

#[test]
fn fuzz_html_injection() {
    scan_source("<script>alert('xss')</script>", "jsx");
}

#[test]
fn fuzz_very_long_strings() {
    let long_str = "x".repeat(50000);
    let source = format!("const s = '{long_str}';");
    scan_source(&source, "jsx");
}

#[test]
fn fuzz_buffer_overflow_attempts() {
    // Various edge cases that might cause buffer overflows
    let cases = [
        "//",
        "/*",
        "*/",
        "//\n",
        "/**/",
        "\"",
        "'",
        "`",
        "\\",
        "\\\\",
        "${",
        "}",
        "<",
        "/>",
        "</",
        "(",
        ")",
        "[",
        "]",
        "=>",
        "...",
        "?.",
        "??",
        "||=",
        "&&=",
        "??=",
        "import.meta",
        "new.target",
        "import()",
        "function*(){}",
        "async()=>{}",
        "yield",
        "await",
        "class{}",
        "static{}",
    ];
    for case in &cases {
        scan_source(case, "jsx");
        scan_source(case, "tsx");
    }
}
