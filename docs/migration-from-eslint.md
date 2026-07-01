# Migrating from ESLint to React Auditor

React Auditor is a drop-in replacement for ESLint focused on React, TypeScript, and web security rules. It runs 5-10x faster than ESLint by leveraging the oxc parser in Rust.

## Quick Start

```bash
# Install
cargo install react-auditor

# Run on your project
cd my-project
react-auditor src/
```

## Config Comparison

### ESLint `.eslintrc.json`
```json
{
  "extends": [
    "eslint:recommended",
    "plugin:react/recommended",
    "plugin:@typescript-eslint/recommended"
  ],
  "rules": {
    "no-console": "warn",
    "no-var": "error",
    "react/jsx-key": "error"
  }
}
```

### React Auditor `.rauditrc.toml`
```toml
"no-console" = "warning"
"no-var" = "error"
"no-missing-key" = "error"
```

Or in `package.json`:
```json
{
  "reactAuditor": {
    "rules": {
      "no-console": "warning",
      "no-var": "error"
    }
  }
}
```

## Rule Mapping

| ESLint Rule | React Auditor Rule | Notes |
|---|---|---|
| `no-console` | `no-console` | Same behavior, includes auto-fix |
| `no-var` | `no-var` | Same behavior, includes auto-fix |
| `max-params` | `max-params` | Same, default max 3 |
| `complexity` | `complexity` | Same default threshold of 10 |
| `no-shadow` | `no-shadow` | Same behavior |
| `no-empty` | `no-empty-blocks` | React Auditor includes try/catch/finally |
| `react/jsx-key` | `no-missing-key` | Same behavior |
| `react/no-array-index-key` | `no-index-key` | Same behavior |
| `react/jsx-no-duplicate-props` | `jsx-no-duplicate-props` | Same behavior |
| `react/no-danger` | `no-dangerously-set-innerhtml` | Identical check |
| `react/button-has-type` | `button-has-type` | Same behavior |
| `react/jsx-no-target-blank` | _(none)_ | Use `rel="noopener noreferrer"` manually |
| `@typescript-eslint/no-explicit-any` | `no-explicit-any` | Stricter — catches `as any` and `<any>` casts |
| `@typescript-eslint/no-non-null-assertion` | `no-non-null-assertion` | Same behavior |
| `@typescript-eslint/no-empty-interface` | `no-empty-interface` | Same behavior |
| `@typescript-eslint/consistent-type-imports` | `consistent-type-imports` | Same behavior |
| `@typescript-eslint/explicit-function-return-type` | `explicit-return-type` | Same behavior |
| `@typescript-eslint/prefer-interface` | `prefer-interface` | Same behavior |
| `jsx-a11y/alt-text` | `img-alt` | Same behavior, **error** by default |
| `jsx-a11y/label-has-associated-control` | `label-associated` | Same behavior |
| `jsx-a11y/heading-has-content` | `heading-levels` | Checks heading level ordering |
| `jsx-a11y/anchor-has-content` | `a-has-content` | Same behavior, includes buttons |
| `jsx-a11y/aria-valid` | `aria-valid` | Same behavior |
| `no-eval` | `no-eval` | Same, also catches `Function()` and `setTimeout(string)` |
| `next/next/no-img-element` | `no-img-element` | Same behavior |
| `next/next/no-script-tag-in-head` | `no-script-tag-in-head` | Same behavior |
| `next/next/no-page-link` | `no-page-link` | Same behavior |

## Unsupported ESLint Rules

These common ESLint rules have no direct equivalent in React Auditor:

| ESLint Rule | Alternative |
|---|---|
| `react/prop-types` | Use TypeScript interfaces instead |
| `react/display-name` | Component names are checked by `consistent-component-naming` |
| `react-hooks/exhaustive-deps` | Covered by `no-missing-deps` with stricter checks |
| `@typescript-eslint/no-unused-vars` | React Auditor has `no-unused-vars` with similar behavior |
| `import/order` | Use `dprint` or `prettier` for import sorting |

## Running with lint-staged

Replace your ESLint config in `package.json`:

```json
{
  "lint-staged": {
    "*.{js,jsx,ts,tsx}": ["react-auditor"]
  }
}
```

## Performance Comparison

| Metric | ESLint | React Auditor |
|---|---|---|
| Cold scan, 100 files (~5k LOC) | ~3-5s | ~0.02s |
| Cold scan, 1000 files (~50k LOC) | ~30-60s | ~0.2s |
| Incremental (single file) | ~0.5-1s | ~0.0001s |
| Binary size | ~50MB (with plugins) | ~8MB |
| Installation | `npm install` (50+ deps) | `cargo install` (single binary) |

## CI Integration

### ESLint
```yaml
- run: npx eslint src/
```

### React Auditor
```yaml
- uses: actions-rs/cargo@v1
  with:
    command: install
    args: react-auditor
- run: react-auditor src/ --fail-on error
```
