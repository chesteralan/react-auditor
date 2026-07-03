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
├── main.rs                    # CLI entry point (clap)
├── lib.rs                     # Module declarations
├── cli.rs                     # Cli struct (clap derive)
├── config.rs                  # Config loader (.rauditrc, package.json)
├── scanner.rs                 # Scanner orchestration + rule runner
├── cache.rs                   # Incremental cache (.raudit-cache.json)
├── watch.rs                   # Watch mode (notify crate)
├── docs.rs                    # Rule documentation generator (--docs)
├── rules/
│   ├── mod.rs                 # Rule trait, Severity, RuleRegistry
│   ├── quality/               # 13 rules
│   ├── react/                 # 17 rules
│   ├── typescript/            # 9 rules
│   ├── security/              # 7 rules
│   ├── nextjs/                # 5 rules
│   └── performance/           # 17 files (5 performance + 11 accessibility)
└── formatters/
    ├── mod.rs
    ├── stylish.rs             # Color-coded terminal output
    ├── json.rs                # Machine-readable JSON
    └── compact.rs             # Single-line per violation
```

## Adding a New Rule

1. Choose the appropriate category under `src/rules/`
2. Create a new file, e.g. `src/rules/react/no_missing_key.rs`
3. Implement the `Rule` trait:

```rust
use oxc_ast::ast::Program;
use oxc_semantic::Semantic;
use crate::rules::{Rule, RuleMeta, RuleFinding, Severity, Fix};

pub struct NoMissingKey;

impl Rule for NoMissingKey {
    fn meta(&self) -> &RuleMeta {
        &RuleMeta {
            id: "no-missing-key",
            default_severity: Severity::Error,
            category: "react",
            description: "List items should have a `key` prop",
        }
    }

    fn run(&self, program: &Program, semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        // check logic using oxc AST types
        Vec::new()
    }
}
```

4. Add the rule to `register_all()` in `src/rules/mod.rs`:
   ```rust
   self.rules.push(Box::new(react::no_missing_key::NoMissingKey));
   ```
5. For auto-fix support, override `has_fix() -> bool` and `fix()` on your rule struct.

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

## Distributing via npm

The npm package (`npm/package.json`) wraps a single pre-built binary. The binary is built during CI publish and copied into `npm/`. `npm/wrapper.js` finds it at `__dirname/react-auditor` or falls back to `which react-auditor`.

Binary targets on release:
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`
