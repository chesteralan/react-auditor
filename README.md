# react-auditor

**A blazing-fast Rust CLI to scan JS/TS/React code for best practices, quality, and security issues.**

Powered by [oxc](https://oxc.rs/) — the same parser used by oxlint and many other tools. Scans thousands of files per second.

## Installation

### npm (recommended for Node projects)

```bash
npm install --save-dev react-auditor
```

The npm package automatically downloads the correct pre-built binary for your platform (macOS, Linux, Windows).

### cargo

```bash
cargo install react-auditor
```

## Quick Start

```bash
# Scan all files in src/ (default)
react-auditor

# Scan specific paths
react-auditor src/ --format stylish

# Output JSON for CI
react-auditor --format json

# Write JSON log and enforce warning limit
react-auditor --log audit.json --max-warnings 10
```

## lint-staged Integration

Add to `package.json`:

```json
{
  "lint-staged": {
    "*.{js,jsx,ts,tsx}": ["react-auditor --log audit-log.json"]
  }
}
```

## husky Integration

```bash
npx husky add .husky/pre-commit "npx lint-staged"
```

## Configuration

Create `.rauditrc.toml`:

```toml
[rules]
no-console = "error"
no-any = "warn"
no-magic-numbers = "off"

log = "audit-results.json"
format = "stylish"
max_warnings = 10
```

Also supports `.rauditrc.json` and `package.json#reactAuditor`.

## Rules (43 total)

| Category | Count | Rule IDs |
|----------|-------|----------|
| Code Quality | 13 | `no-console`, `no-empty-blocks`, `no-var`, `max-params`, `no-long-functions`, `prefer-early-return`, `no-commented-code`, `no-deep-nesting`, `no-magic-numbers`, `consistent-return`, `no-unused-vars`, `no-shadow`, `complexity` |
| React | 13 | `no-missing-key`, `no-inline-styles`, `consistent-component-naming`, `no-index-key`, `no-inline-functions`, `prefer-function-components`, `no-unnecessary-memo`, `no-multiple-render-methods`, `no-side-effects-in-render`, `hook-rules`, `effect-deps-complete`, `no-set-state-in-effect`, `no-set-state-in-render` |
| TypeScript | 8 | `no-any`, `no-non-null-assertion`, `no-type-assertion`, `no-empty-interface`, `consistent-type-imports`, `explicit-return-type`, `strict-null-checks`, `prefer-interface` |
| Security | 6 | `no-dangerously-set-innerhtml`, `no-eval`, `no-script-url`, `no-hardcoded-secrets`, `no-unsanitized-input`, `no-insecure-protocol` |
| Performance | 5 | `prefer-fragments`, `no-bind-in-jsx`, `no-heavy-computation-in-render`, `lazy-load-components`, `no-heavy-computation-in-render` |
| Accessibility | 4 | `img-alt`, `button-has-type`, `aria-valid`, `heading-levels`, `label-associated` |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | No errors (warnings may exist if below `maxWarnings`) |
| 1 | Errors found or warnings exceed `maxWarnings` |

## License

MIT
