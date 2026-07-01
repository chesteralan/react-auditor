# Roadmap

## Phase 1 — Foundation ✅
- [x] Initialize Rust project with `cargo init`
- [x] Set up `clap` CLI argument parser
- [x] Integrate `oxc_parser` for JS/TS/JSX/TSX parsing
- [x] Set up configuration loader (TOML + JSON + `package.json`)
- [x] Implement basic file globbing and file reading
- [x] Set up CI (GitHub Actions: build, test, clippy, fmt)

## Phase 2 — Rule Engine ✅
- [x] Define `Rule` trait with `meta()` and `run()` methods
- [x] Implement `RuleContext` and `Violation` types
- [x] Build rule registry (discover and run all registered rules)
- [x] Implement severity levels (error / warn / off)
- [x] Wire rule filtering (`--rules` flag, config-based enable/disable)

## Phase 3 — Output & Formatting ✅
- [x] Implement `stylish` terminal formatter with colors (`termcolor`)
- [x] Implement `json` formatter for machine-readable output
- [x] Implement `compact` single-line formatter
- [x] Add `--log <path>` for file output
- [x] Add `--max-warnings` and `--quiet` flags
- [x] Add `--fix` infrastructure (auto-fixable rule support)

## Phase 4 — Rules: Code Quality ✅
- [x] `no-console`
- [x] `no-empty-blocks`
- [x] `no-var`
- [x] `max-params`
- [x] `no-long-functions`
- [x] `prefer-early-return`
- [x] `no-commented-code`
- [x] `no-deep-nesting`
- [x] `no-magic-numbers`
- [x] `consistent-return`
- [x] `no-unused-vars`
- [x] `no-shadow`
- [x] `complexity`

## Phase 5 — Rules: React ✅
- [x] `no-missing-key`
- [x] `no-inline-styles`
- [x] `consistent-component-naming`
- [x] `no-index-key`
- [x] `no-inline-functions` (in JSX props)
- [x] `prefer-function-components`
- [x] `no-unnecessary-memo`
- [x] `no-multiple-render-methods`
- [x] `no-side-effects-in-render`
- [x] `hook-rules` (Rules of Hooks)
- [x] `effect-deps-complete`
- [x] `no-set-state-in-effect`
- [x] `no-set-state-in-render`

## Phase 6 — Rules: TypeScript ✅
- [x] `no-any`
- [x] `no-non-null-assertion`
- [x] `no-type-assertion`
- [x] `no-empty-interface`
- [x] `consistent-type-imports`
- [x] `explicit-return-type`
- [x] `strict-null-checks`
- [x] `prefer-interface` / `prefer-type-alias`

## Phase 7 — Rules: Security ✅
- [x] `no-dangerously-set-innerhtml`
- [x] `no-eval`
- [x] `no-script-url`
- [x] `no-hardcoded-secrets`
- [x] `no-unsanitized-input`
- [x] `no-insecure-protocol`

## Phase 8 — Rules: Performance & Accessibility ✅
- [x] `prefer-fragments`
- [x] `no-bind-in-jsx`
- [x] `img-alt`
- [x] `button-has-type`
- [x] `label-associated`
- [x] `no-heavy-computation-in-render`
- [x] `lazy-load-components`
- [x] `aria-valid`
- [x] `heading-levels`

## Phase 9 — Integration & Distribution ✅
- [x] Build npm wrapper package (platform-specific binary downloads)
- [x] Write postinstall script for npm package (`install.js`)
- [x] Add GitHub Releases workflow (build for macOS, Linux, Windows)
- [x] Document `lint-staged` / `husky` setup in README
- [x] Write comprehensive help text (`--help`)

## Phase 10 — Polish ✅
- [x] End-to-end tests on real-world React codebases
- [x] Performance benchmarks (criterion)
- [x] VS Code extension scaffold
- [x] Auto-fix support for common rules (infrastructure + `no-var` fix)

## Phase 11 — Publishing & CI/CD ✅
- [x] npm publish step in GitHub Actions release workflow
- [x] Publish npm package to `registry.npmjs.org` (triggered by `v*` tags)
- [x] VS Code extension publish workflow (`.github/workflows/publish-vscode.yml`)
- [x] VS Code extension: live diagnostics, status bar, debounced file-watching
- [ ] Tag a `v0.1.0` release to trigger npm + VS Code publishing
