# react-auditor

**A blazing-fast Rust CLI to scan JS/TS/React code for best practices, quality, security, performance, Next.js, and accessibility issues.**

Powered by [oxc](https://oxc.rs/) — parallel scanning at ~138µs per 1k LOC. 62 rules across 7 categories.

## Installation

```bash
# npm (recommended for Node projects)
npm install --save-dev react-auditor

# Cargo
cargo install react-auditor

# Homebrew
brew install react-auditor

# Docker
docker run ghcr.io/chesteralan/react-auditor src/
```

The npm package automatically downloads the correct pre-built binary for your platform (macOS ARM/x64, Linux ARM/x64, Windows x64).

## Quick Start

```bash
# Scan all files in src/ (default)
react-auditor

# Scan specific paths with JSON output
react-auditor src/ --format json

# Run only specific categories
react-auditor --category react,typescript

# Auto-fix where supported
react-auditor --fix

# Watch mode (re-scan on file changes)
react-auditor -W

# Fail CI on errors only
react-auditor --fail-on error

# Write JSON log
react-auditor --log audit.json
```

## Configuration

Create `.rauditrc.toml` in your project root:

```toml
"no-console" = "warning"
"no-var" = "error"
"no-magic-numbers" = "off"

# Multi-root workspace
workspaces = ["packages/*"]

# Per-file-type rule overrides
[file_types]
".jsx" = { "no-explicit-any" = "off" }
```

Also supports `.rauditrc.json` and `package.json#reactAuditor`.

## Features

| Feature | Description |
|---------|-------------|
| Parallel scanning | Rayon-based concurrent file processing |
| Incremental cache | Only re-scans changed files (`.raudit-cache.json`) |
| Watch mode | `-W` / `--watch` via `notify` crate with 200ms debounce |
| Multi-root workspace | `workspaces` config field expands globs to scan roots |
| Per-file-type overrides | `file_types` config maps extension to rule overrides |
| Auto-fix | `--fix` for `no-console`, `no-var`, `no-empty-blocks` |
| --fail-on | Exit code based on severity threshold |
| --category | Filter rules by category |
| --ignore | Skip files/directories by pattern |
| Formatters | `stylish` (default), `json`, `compact` |
| Progress bar | indicatif spinner during scans |

## Output Formats

```bash
react-auditor --format stylish   # colored, human-readable (default)
react-auditor --format json      # machine-readable for CI
react-auditor --format compact   # one-line per violation
react-auditor --quiet            # errors only, no warnings
```

## Rules (62 total)

| Category | Count | Rule IDs |
|----------|-------|----------|
| Code Quality | 13 | `no-console`, `no-empty-blocks`, `no-var`, `max-params`, `no-long-functions`, `prefer-early-return`, `no-commented-code`, `no-deep-nesting`, `no-magic-numbers`, `consistent-return`, `no-unused-vars`, `no-shadow`, `complexity` |
| React | 16 | `no-missing-key`, `no-inline-styles`, `consistent-component-naming`, `no-index-key`, `no-inline-functions`, `prefer-function-components`, `no-unnecessary-memo`, `no-multiple-render-methods`, `no-side-effects-in-render`, `hook-rules`, `no-missing-deps`, `no-set-state-in-effect`, `no-set-state-in-render`, `jsx-no-duplicate-props`, `no-direct-mutation`, `no-ref-in-component-name` |
| TypeScript | 9 | `no-any`, `no-non-null-assertion`, `no-type-assertion`, `no-empty-interface`, `consistent-type-imports`, `explicit-return-type`, `strict-null-checks`, `prefer-interface`, `no-explicit-any` |
| Security | 7 | `no-dangerously-set-innerhtml`, `no-eval`, `no-script-url`, `no-hardcoded-secrets`, `no-unsanitized-input`, `no-insecure-protocol`, `no-unsafe-iframe` |
| Accessibility | 7 | `img-alt`, `button-has-type`, `label-associated`, `aria-valid`, `heading-levels`, `a-has-content`, `no-ambiguous-labels` |
| Performance | 5 | `prefer-fragments`, `no-bind-in-jsx`, `no-heavy-computation-in-render`, `lazy-load-components`, `no-large-libraries` |
| Next.js | 5 | `no-img-element`, `no-script-tag-in-head`, `no-page-link`, `no-head-element`, `no-sync-script` |

## Pre-commit Integration

```json
{
  "lint-staged": {
    "*.{js,jsx,ts,tsx}": ["react-auditor --quiet --max-warnings 0"]
  }
}
```

## VS Code Extension

Install from the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=chesteralan.react-auditor) for live diagnostics, quick-fixes, gutter decorations, and configuration UI.

## GitHub Action

```yaml
jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v7
      - uses: ./.github/actions/react-auditor
        with:
          args: "src/ --fail-on error"
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | No errors (warnings may exist) |
| 1 | Errors found or `--fail-on` threshold exceeded |

## Performance Benchmarks

| Scenario | Time |
|----------|------|
| 1k LOC (clean) | ~138µs |
| 10k LOC (clean) | ~154µs |
| Cold scan, 100 files | ~0.02s |
| Incremental (1 file) | ~0.0001s |

## License

MIT
