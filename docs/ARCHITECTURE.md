# Architecture

## Tech Stack

- **Language:** Rust
- **Parser:** [oxc](https://docs.rs/oxc) (Oxidation Compiler) — Rust-native JS/TS parser with JSX and TypeScript support
- **AST Traversal:** oxc's visitor pattern
- **CLI Framework:** `clap`
- **Config:** `serde` + `toml`/`json` deserialization
- **Output:** `serde_json` for JSON logs, `termcolor` for terminal coloring

## Overview

```
┌──────────────────────────────────────┐
│            CLI Entry (clap)           │
│         src/bin/react-auditor.rs      │
├──────────────────────────────────────┤
│           Config Loader               │
│    .rauditrc.toml / .rauditrc.json    │
│    package.json / CLI flags           │
├──────────────────────────────────────┤
│           Scanner Engine              │
│  ┌──────────┐ ┌──────────┐ ┌──────┐  │
│  │  Parser  │ │  Walker  │ │ Rule │  │
│  │ (oxc)    │ │ (oxc     │ │Runner│  │
│  │          │ │  visit)  │ │      │  │
│  └──────────┘ └──────────┘ └──────┘  │
├──────────────────────────────────────┤
│          Results Formatter            │
│  ┌──────────┐ ┌──────────┐           │
│  │ Terminal │ │  File    │           │
│  │ Output   │ │  Log     │           │
│  └──────────┘ └──────────┘           │
└──────────────────────────────────────┘
```

## Module Design

### CLI Entry (`src/bin/`)
- Built with `clap` for argument parsing
- Accepts file paths, glob patterns, and flags
- Reads stdin for `lint-staged` integration (list of staged files)

### Config Loader (`src/config/`)
- Loads configuration from (highest priority first):
  1. CLI flags
  2. `.rauditrc.toml` / `.rauditrc.json` (project-local config)
  3. `"reactAuditor"` key in `package.json` (for Node project integration)
- Config includes: enabled rules, rule severity levels, log file path, output format

### Scanner Engine (`src/scanner/`)

- **Parser**: Uses `oxc_parser` to parse JS/JSX/TS/TSX files into an AST
- **Walker**: Uses `oxc_semantic` + custom traversal to walk the AST with semantic information (scope, types, references)
- **Rule Runner**: Iterates over all registered rules and invokes their check logic, passing the AST + semantic data

### Rule System (`src/rules/`)

- Each rule is a Rust module (struct + trait impl)
- Rules implement a common trait:

```rust
pub trait Rule {
    fn meta(&self) -> RuleMeta;
    fn run<'a>(&self, node: &'a AstNode<'a>, ctx: &mut RuleContext<'a>);
}
```

- Rules emit violations via `ctx.add_violation(Violation { file, line, column, rule_id, message, severity })`

### Results Formatter (`src/formatters/`)

| Formatter | Description |
|-----------|-------------|
| `stylish` | Color-coded terminal output (default, similar to ESLint stylish) |
| `json` | Machine-readable JSON output |
| `compact` | Single-line-per-violation compact format |

## Data Flow

1. CLI receives file paths (from args or stdin)
2. Config loader resolves effective configuration
3. For each file:
   a. Read file content
   b. Parse with `oxc_parser` into AST
   c. Walk AST and run each rule's visitor
   d. Collect violations
4. Aggregate violations across all files
5. Format and output results (terminal + optional file)
6. Exit with code: 0 (no errors), 1 (errors found)

## Distribution

The tool compiles to a single static binary. Distribution options:

- **npm package** (like `@biomejs/biome`): thin npm wrapper downloads the correct platform binary post-install
- **GitHub Releases**: direct download for CI pipelines
- **cargo install**: `cargo install react-auditor`
