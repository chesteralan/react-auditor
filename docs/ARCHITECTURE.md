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
┌─────────────────────────────────────────┐
│           CLI Entry (clap derive)        │
│              src/main.rs                 │
├─────────────────────────────────────────┤
│              Config Loader               │
│       .rauditrc.toml / .rauditrc.json    │
│       package.json / CLI flags           │
├─────────────────────────────────────────┤
│              Scanner Engine              │
│  ┌──────────────────────────────────┐    │
│  │  Cache check → Parse → Run      │    │
│  │  rules → Collect violations      │    │
│  │  (rayon parallel files)          │    │
│  └──────────────────────────────────┘    │
│  Optional: Watch mode (notify crate)     │
├─────────────────────────────────────────┤
│          Results Formatter               │
│  ┌──────────┐ ┌──────────┐ ┌─────────┐  │
│  │ stylish  │ │   json   │ │ compact │  │
│  │ (colored)│ │ (machine)│ │(oneline)│  │
│  └──────────┘ └──────────┘ └─────────┘  │
└─────────────────────────────────────────┘
```

## Module Design

### CLI Entry (`src/main.rs`)
- Built with `clap` derive for argument parsing (`src/cli.rs`)
- Accepts file paths, glob patterns, and flags
- Integrates cache, watch mode, and `--docs` generation

### Config Loader (`src/config.rs`)
- Loads configuration from (highest priority first):
  1. CLI flags
  2. `.rauditrc.toml` / `.rauditrc.json` (project-local config)
  3. `"reactAuditor"` key in `package.json` (for Node project integration)
- Config includes: enabled rules, rule severity levels, log file path, output format, workspaces, file type overrides

### Scanner Engine (`src/scanner.rs`)

- **Parsing**: Uses `oxc_parser` to parse JS/JSX/TS/TSX files into AST
- **Semantic**: Uses `oxc_semantic` for scope, type, and reference analysis
- **Caching**: `src/cache.rs` stores file mtime hashes in `.raudit-cache.json` to skip unchanged files
- **Rule Runner**: Iterates over all registered rules, passes `Program` + `Semantic` + source text, collects `RuleFinding`s
- **Parallelism**: `rayon` parallel iterator for concurrent file processing
- **Watch**: `src/watch.rs` uses `notify` crate with 200ms debounce

### Rule System (`src/rules/`)

- Each rule is a Rust struct in its own module file
- Rules implement the `Rule` trait:

```rust
pub trait Rule: Send + Sync {
    fn meta(&self) -> &RuleMeta;
    fn run(&self, program: &Program, semantic: &Semantic, source_text: &str) -> Vec<RuleFinding>;
    fn fix(&self, finding: &RuleFinding, source_text: &str) -> Option<Fix> { None }
    fn has_fix(&self) -> bool { false }
}
```

- Rules return `Vec<RuleFinding>` — violations are aggregated by `RuleRegistry::run_rules()`
- Severity is resolved from config overrides or rule default in `run_rules()`
- 6 submodules: `quality/`, `react/`, `typescript/`, `security/`, `nextjs/`, `performance/` (contains both performance and accessibility rules by category)

### Results Formatter (`src/formatters/`)

| Formatter | Description |
|-----------|-------------|
| `stylish` | Color-coded terminal output (default, similar to ESLint stylish) |
| `json` | Machine-readable JSON output |
| `compact` | Single-line-per-violation compact format |

## Data Flow

1. CLI parses args; loads config from file/package.json
2. Resolves file globs, filter categories, severity overrides
3. For each file (parallel via `rayon`):
   a. Check cache — skip if unchanged (unless `--no-cache`)
   b. Read file content
   c. Parse with `oxc_parser` into `Program` AST
   d. Build `Semantic` from AST
   e. Run all enabled rules, collecting `RuleFinding`s
   f. Convert to `Violation` with file path + resolved severity
4. Aggregate violations across all files
5. Apply `--fix` if requested (in-place file edits)
6. Format and output results (terminal + optional file log)
7. Exit with code: 0 (no errors below threshold), 1 (errors found)

## Distribution

The tool compiles to a single static binary. Distribution options:

- **npm package**: thin npm wrapper bundles the binary (`npm/package.json` + `npm/wrapper.js`), published via CI on release
- **cargo install**: `cargo install react-auditor` — published via CI on release
- **Homebrew**: formula auto-published to `chesteralan/homebrew-tap` via CI on release
- **VS Code extension** (`react-auditor.vsix`): published via CI on release, binary bundled in the extension
- **Docker**: `docker/Dockerfile` — multi-stage build with `debian:bookworm-slim`
- **GitHub Releases**: direct download for CI pipelines (5 platform targets)
- **GitHub Action**: `.github/actions/react-auditor/` — composite action with problem matcher
