use std::collections::HashMap;
use std::path::PathBuf;

use criterion::{Criterion, criterion_group, criterion_main};

use react_auditor::scanner::Scanner;

fn fixture_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests/fixtures");
    p.push(name);
    p
}

fn bench_scan_all_fixtures(c: &mut Criterion) {
    let fixtures = vec![
        "has_issues.jsx",
        "typescript_issues.ts",
        "security_issues.jsx",
        "performance_issues.jsx",
        "accessibility_issues.jsx",
    ];

    let mut paths = Vec::new();
    for f in &fixtures {
        paths.push(fixture_path(f).to_string_lossy().to_string());
    }

    c.bench_function("scan_all_fixtures", |b| {
        b.iter(|| {
            let scanner = Scanner::new(paths.clone(), HashMap::new());
            let results = scanner.scan().unwrap();
            let total: usize = results.iter().map(|r| r.violations.len()).sum();
            std::hint::black_box(total);
        })
    });
}

fn bench_scan_large_generated(c: &mut Criterion) {
    let content = generate_large_file(1000);

    let path = std::env::temp_dir().join("_bench_large.tsx");
    std::fs::write(&path, &content).unwrap();

    c.bench_function("scan_1000_line_tsx", |b| {
        b.iter(|| {
            let scanner = Scanner::new(vec![path.to_string_lossy().to_string()], HashMap::new());
            let results = scanner.scan().unwrap();
            let total: usize = results.iter().map(|r| r.violations.len()).sum();
            std::hint::black_box(total);
        })
    });

    let _ = std::fs::remove_file(&path);
}

fn generate_large_file(num_components: usize) -> String {
    let mut content = String::from("import React from 'react';\n\n");
    for i in 0..num_components {
        content.push_str(&format!(
            r#"
function Component{i}({{ name, items }}) {{
  console.log('component {i}');
  var x = 1;
  return (
    <div style={{{{ color: 'red' }}}}>
      <h1>{{name}}</h1>
      {{items.map(item => (
        <div key={{item.id}}>{{item.name}}</div>
      ))}}
    </div>
  );
}}
"#
        ));
    }
    content.push_str("export default Component0;\n");
    content
}

criterion_group!(benches, bench_scan_all_fixtures, bench_scan_large_generated);
criterion_main!(benches);
