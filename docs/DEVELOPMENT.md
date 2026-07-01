# Development

## Prerequisites

- Rust toolchain (latest stable) — install via `rustup`
- `cargo`

## Setup

```bash
git clone <repo-url>
cd react-implementation-auditor
cargo build
```

## Project Structure

```
src/
├── bin/
│   └── react-auditor.rs       # CLI entry point (clap)
├── config/
│   ├── mod.rs                 # Config loader
│   └── types.rs               # Config types (serde)
├── scanner/
│   ├── mod.rs                 # Scanner orchestration
│   ├── parser.rs              # oxc_parser wrapper
│   └── walker.rs              # AST traversal driver
├── rules/
│   ├── mod.rs                 # Rule registry
│   ├── quality/
│   │   ├── no_console.rs
│   │   ├── no_unused_vars.rs
│   │   └── ...
│   ├── react/
│   │   ├── no_missing_key.rs
│   │   ├── effect_deps.rs
│   │   └── ...
│   ├── typescript/
│   ├── security/
│   ├── performance/
│   ├── accessibility/
│   └── testing/
├── formatters/
│   ├── mod.rs
│   ├── stylish.rs
│   ├── json.rs
│   └── compact.rs
└── utils/
    └── mod.rs
```

## Adding a New Rule

1. Choose the appropriate category under `src/rules/`
2. Create a new file, e.g. `src/rules/react/no_missing_key.rs`
3. Implement the `Rule` trait:

```rust
use crate::scanner::{Rule, RuleContext, RuleMeta, Violation};
use oxc_ast::AstNode;

pub struct NoMissingKey;

impl Rule for NoMissingKey {
    fn meta(&self) -> RuleMeta {
        RuleMeta {
            id: "no-missing-key",
            severity: Severity::Error,
            category: "react",
            description: "List items should have a `key` prop",
        }
    }

    fn run<'a>(&self, node: &'a AstNode<'a>, ctx: &mut RuleContext<'a>) {
        // check logic using oxc AST types
    }
}
```

4. Register the rule in `src/rules/mod.rs`

## Testing

```bash
cargo test
cargo test -- --nocapture  # show output
```

Test fixtures live in `tests/fixtures/`.

## Building

```bash
cargo build --release
```

Outputs to `target/release/react-auditor`.

## Linting

```bash
cargo clippy
cargo fmt
```

## Distributing via npm (optional)

To integrate with `lint-staged` without requiring users to install Rust, publish a thin npm wrapper:

```
react-auditor/
├── Cargo.toml
├── npm/
│   ├── darwin-arm64/   (binary for Apple Silicon)
│   ├── darwin-x64/     (binary for Intel Mac)
│   ├── linux-x64/      (binary for Linux)
│   └── win32-x64/      (binary for Windows)
├── package.json        (optional npm wrapper)
└── src/
```

This follows the pattern used by Biome, esbuild, and SWC.
