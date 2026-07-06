use std::collections::HashMap;
use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

use react_auditor::scanner::Scanner;

/// Generate a source file with `stmt_count` statements.
fn generate_source(stmt_count: usize) -> String {
    let mut source = String::from("import React from 'react';\n\n");
    source.push_str("const App: React.FC = () => {\n");
    source.push_str("  const [count, setCount] = React.useState(0);\n");
    source.push_str("  const [items, setItems] = React.useState<string[]>([]);\n\n");

    for i in 0..stmt_count {
        source.push_str(&format!(
            "  const item_{i} = {{ id: {i}, name: 'test_{i}', value: {i} * 2 }};\n"
        ));
    }

    source.push_str("\n  return (\n    <div>\n");
    source.push_str("      <h1>Hello</h1>\n");
    source.push_str("      <ul>\n");
    for i in 0..(stmt_count / 10).max(1) {
        source.push_str(&format!("        <li key={i}>{{item_{i}.name}}</li>\n"));
    }
    source.push_str("      </ul>\n");
    source.push_str("    </div>\n");
    source.push_str("  );\n};\n\nexport default App;\n");
    source
}

fn bench_scan_1k_loc(c: &mut Criterion) {
    let source = generate_source(200); // ~1k LOC
    let dir = std::env::temp_dir().join("_bench_scan");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("bench_1k.tsx");
    std::fs::write(&path, &source).unwrap();

    c.bench_function("scan_1k_loc", |b| {
        b.iter(|| {
            let scanner = Scanner::new(
                black_box(vec![path.to_string_lossy().to_string()]),
                black_box(HashMap::new()),
                black_box(None),
                black_box(Vec::new()),
            );
            let _ = scanner.scan();
        })
    });

    let _ = std::fs::remove_file(&path);
}

fn bench_scan_10k_loc(c: &mut Criterion) {
    let source = generate_source(2000); // ~10k LOC
    let dir = std::env::temp_dir().join("_bench_scan");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("bench_10k.tsx");
    std::fs::write(&path, &source).unwrap();

    c.bench_function("scan_10k_loc", |b| {
        b.iter(|| {
            let scanner = Scanner::new(
                black_box(vec![path.to_string_lossy().to_string()]),
                black_box(HashMap::new()),
                black_box(None),
                black_box(Vec::new()),
            );
            let _ = scanner.scan();
        })
    });

    let _ = std::fs::remove_file(&path);
}

fn bench_scan_clean_file(c: &mut Criterion) {
    // A clean file that shouldn't trigger any rules
    let source = r#"
import React from 'react';

interface Props {
  name: string;
}

const Greeting: React.FC<Props> = ({ name }) => {
  return <div>Hello, {name}!</div>;
};

export default Greeting;
"#;
    let dir = std::env::temp_dir().join("_bench_scan");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("bench_clean.tsx");
    std::fs::write(&path, source).unwrap();

    c.bench_function("scan_clean_file", |b| {
        b.iter(|| {
            let scanner = Scanner::new(
                black_box(vec![path.to_string_lossy().to_string()]),
                black_box(HashMap::new()),
                black_box(None),
                black_box(Vec::new()),
            );
            let _ = scanner.scan();
        })
    });

    let _ = std::fs::remove_file(&path);
}

criterion_group!(
    benches,
    bench_scan_1k_loc,
    bench_scan_10k_loc,
    bench_scan_clean_file
);
criterion_main!(benches);
