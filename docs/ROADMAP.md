# Roadmap

> Current version: **v0.3.2-dev** · 70 rules across 8 categories · Active branch: `feat/v0.3.2`

## Phase 1 — Foundation ✅
- [x] Initialize Rust project with `cargo init`
- [x] Set up `clap` CLI argument parser
- [x] Integrate `oxc_parser` for JS/TS/JSX/TSX parsing
- [x] Set up configuration loader (TOML + JSON + `package.json`)
- [x] Implement basic file globbing and file reading
- [x] Set up CI (GitHub Actions: build, test, clippy, fmt)

## Phase 2 — Rule Engine ✅
- [x] Define `Rule` trait with `meta()` and `run()` methods
- [x] Implement `Violation` and `RuleFinding` types
- [x] Build rule registry (`RuleRegistry`) with `register_all()`
- [x] Implement severity levels (error / warn / off)
- [x] Wire rule filtering (`--rules` flag, config-based enable/disable)
- [x] Auto-fix infrastructure: `has_fix()` + `fix()` on `Rule` trait

## Phase 3 — Output & Formatting ✅
- [x] Implement `stylish` terminal formatter with colors (`termcolor`)
- [x] Implement `json` formatter for machine-readable output
- [x] Implement `compact` single-line formatter
- [x] Add `--log <path>` for file output
- [x] Add `--max-warnings` and `--quiet` flags
- [x] Add `--fix` infrastructure (auto-fixable rule support)
- [x] Snapshot tests for all three formatters

## Phase 4 — Rules: Code Quality ✅ (13 rules)
- [x] `no-console` · `no-empty-blocks` · `no-var`
- [x] `max-params` · `no-long-functions` · `prefer-early-return`
- [x] `no-commented-code` · `no-deep-nesting` · `no-magic-numbers`
- [x] `consistent-return` · `no-unused-vars` · `no-shadow`
- [x] `complexity`

## Phase 5 — Rules: React ✅ (19 rules)
- [x] `no-missing-key` · `no-inline-styles` · `consistent-component-naming`
- [x] `no-index-key` · `no-inline-functions` (in JSX props)
- [x] `prefer-function-components` · `no-unnecessary-memo`
- [x] `no-multiple-render-methods` · `no-side-effects-in-render`
- [x] `hook-rules` (Rules of Hooks) · `no-missing-deps`
- [x] `no-set-state-in-effect` · `no-set-state-in-render`
- [x] `no-direct-mutation` · `jsx-no-duplicate-props`
- [x] `no-ref-in-component-name` · `no-forward-ref`
- [x] `no-array-index-key` · `no-state-in-default-props`

## Phase 6 — Rules: TypeScript ✅ (9 rules)
- [x] `no-any` · `no-non-null-assertion` · `no-type-assertion`
- [x] `no-empty-interface` · `consistent-type-imports`
- [x] `explicit-return-type` · `strict-null-checks`
- [x] `prefer-interface` · `no-explicit-any` (stricter catch)

## Phase 7 — Rules: Security ✅ (7 rules)
- [x] `no-dangerously-set-innerhtml` · `no-eval` · `no-script-url`
- [x] `no-hardcoded-secrets` · `no-unsanitized-input`
- [x] `no-insecure-protocol` · `no-unsafe-iframe`

## Phase 8 — Rules: Performance & Accessibility ✅ (16 rules)
- [x] **Performance (5):** `prefer-fragments` · `no-bind-in-jsx` · `no-heavy-computation-in-render` · `lazy-load-components` · `no-large-libraries`
- [x] **Accessibility (11):** `img-alt` · `button-has-type` · `label-associated` · `aria-valid` · `heading-levels` · `a-has-content` · `no-ambiguous-labels` · `tabindex-no-positive` · `click-events-have-key-events` · `html-has-lang` · `no-autofocus`

## Phase 9 — Rules: Testing ✅ (1 rule)
- [x] `no-skipped-tests` — flag `it.skip`, `describe.skip`, `xit`, `xdescribe` in test files

## Phase 10 — npm Package ✅
- [x] Initial design: platform-specific binary downloads via `install.js` postinstall (archived)
- [x] **Current approach (v0.1.9+):** bundled binary via `wrapper.js` — no GitHub download at install time
- [x] `npm/package.json`: `"bin": "./wrapper.js"`, no scripts, no postinstall
- [x] `npm/wrapper.js`: finds binary at `__dirname/react-auditor` or via `which`, execs via `execFileSync`
- [x] CI publish workflow builds `x86_64-apple-darwin` binary and copies to `npm/` before `npm publish`
- [x] Binary bundled inside npm package (single-arch per publish)
- [x] `install.js` removed, previous `npm/react-auditor` shell shim deleted

## Phase 11 — Polish ✅
- [x] End-to-end tests (16 e2e tests in `tests/e2e_test.rs`)
- [x] Performance benchmarks via `criterion` (`benches/auditor_bench.rs`)
- [x] VS Code extension scaffold + `.vsix` packaging
- [x] Auto-fix support: `no-var`, `no-console`, `no-empty-blocks`, `prefer-fragments` (4 rules with `has_fix() + fix()`)
- [x] `contains_semver()` helper in e2e tests — version-agnostic for future bumps
- [x] VSIX: `*.vsix` gitignored, `vscode/LICENSE` added to fix vsce packaging warning

## Phase 12 — Publishing & CI/CD ✅
- [x] **npm publish** (`.github/workflows/publish-npm.yml`): on `release: [published]` — builds binary + copies to `npm/` + publishes
- [x] **cargo publish** (`.github/workflows/publish-cargo.yml`): on `release: [published]`
- [x] **VS Code publish** (`.github/workflows/publish-vscode.yml`): on `release: [published]` — bundles binary in `vscode/` + `vsce publish`
- [x] **Homebrew publish** (`.github/workflows/publish-homebrew.yml`): `workflow_run` on Release completion + `workflow_dispatch` manual — downloads archives, SHA256, pushes formula to `chesteralan/homebrew-tap`
- [x] **Release build** (`.github/workflows/release.yml`): on `release: [published]` — 5 targets: `x86_64`/`aarch64` Linux + macOS, `x86_64` Windows
- [x] **All publish triggers** unified to `release: [published]` (was `push: tags`) — ensures release artifacts exist before publish runs
- [x] **release-please** (`.github/workflows/release-please.yml`): auto-bumps `Cargo.toml`, `npm/package.json`, `vscode/package.json`; generates changelog from conventional commits

## Phase 13 — Next.js Rules ✅ (5 rules)
- [x] `no-img-element` (use `<Image />` instead of `<img>`)
- [x] `no-script-tag-in-head` (use `<Script />` instead of `<script>` inside `<Head>`)
- [x] `no-page-link` (use `<Link />` instead of `<a>` for internal navigation)
- [x] `no-head-element` (use `<Head />` instead of `<head>`)
- [x] `no-sync-script` (flag synchronous `<Script>` without `strategy="afterInteractive"`)

## Phase 14 — CLI & UX ✅
- [x] **`--rules` flag wired into Scanner.** `Scanner.run_rules()` accepts `category_filter`, CLI passes `rules` through
- [x] **`--ignore` patterns.** Comma-separated globs — skip `node_modules`, `dist/`, `build/`, etc.
- [x] **`--fail-on` severity.** e.g. `--fail-on error` exits 1 only on errors
- [x] **`--no-cache`.** Disable incremental caching (force re-scan all files)
- [x] **`--watch` / `-W`.** Watch mode via `notify` crate with 200ms debounce (`src/watch.rs`)
- [x] **`--docs`.** Generate rule documentation `docs/rules/` per rule (moved from `src/bin/docgen.rs` to `src/docs.rs`)
- [x] **Colored output for `stylish` formatter.** `termcolor` for red/yellow/cyan severity indicators
- [x] **Show rule category in output.** `[react/no-missing-key]` — category is prefixed
- [x] **Spinner/progress bar.** `indicatif` `ProgressBar` during scan

## Phase 15 — Scanner & Engine ✅
- [x] **Parallel file scanning.** `rayon::prelude::par_iter()` — concurrent file processing
- [x] **Incremental / cached scanning.** `.raudit-cache.json` — file mtime hashes, skip unmodified files
- [x] **Watch mode.** `src/watch.rs` via `notify` crate, 200ms debounce
- [x] **Multi-root workspace support.** `config.workspaces` field — expand workspace globs into scan roots
- [x] **Configurable rule defaults per file type.** `file_types` config maps extension → rule overrides
- [x] **`--fix` integrated into Scanner.** Fixes applied in-place after scan

## Phase 16 — Testing ✅
- [x] **69 tests total:** 16 e2e, 5 formatter, 8 integration, 28 proptest, 12 scanner (all pass)
- [x] **Fuzz testing harness.** `fuzz/` directory with `cargo-fuzz` target — parses random bytes + runs all rules (requires Rust nightly to execute)
- [x] **Snapshot testing.** Formatter fixtures compared against `tests/snapshots/` for `stylish`/`json`/`compact`
- [x] **Property-based testing.** `proptest` generates random AST fragments — verify rules don't panic
- [x] **Integration tests with real-world project.** `tests/real-project/` — multi-file project scanned end-to-end
- [x] **Benchmark suite.** `criterion` — ~138µs per 1k LOC (bench results in `benches/auditor_bench.rs`)
- [x] **Version-agnostic test helpers.** `contains_semver()` in e2e tests — no updates needed on version bump

## Phase 17 — VS Code Extension ✅
- [x] **Extension scaffold.** `vscode/extension.js` — activates on JS/TS/JSX/TSX files
- [x] **Live diagnostics.** Debounced file-watching, runs binary, populates Problems panel
- [x] **Binary discovery.** Tries bundled `path.join(__dirname, 'react-auditor')`, falls back to PATH
- [x] **Quick-fix code actions.** `ReactAuditorFixProvider` — "Fix with react-auditor" and "Disable rule" in Problems panel
- [x] **Configuration UI webview.** `react-auditor.configure` command opens webview with rule dropdowns, generates `.rauditrc.toml`
- [x] **Progress notification.** `withProgress()` API for workspace scans with cancellable progress bar
- [x] **Decorations.** Gutter highlight and overview ruler markers for error/warning lines via `updateDecorations()`
- [x] **Status bar.** Clickable status bar item with issue count, spinner during scan
- [x] **VSIX packaging.** `npx vsce package` — `vscode/` dir packaged, `vscode/LICENSE` included
- [x] **Publish workflow.** `.github/workflows/publish-vscode.yml` — bundles binary, packages, publishes to marketplace
- [x] **`.vscodeignore`.** Excludes `node_modules/`, `src/`, `.gitignore`, `*.toml`, `*.lock`

## Phase 18 — Documentation ✅
- [x] **Rule docs generator.** `react-auditor --docs` generates per-rule markdown in `docs/rules/` from `RuleMeta` — includes category, severity, auto-fix status (replaced deprecated `src/bin/docgen.rs`)
- [x] **Website / playground.** `docs/playground.html` — client-side HTML playground with built-in examples (basic, hooks, security, TypeScript, Next.js) + offline mock detection
- [x] **Migration guide.** `docs/migration-from-eslint.md` — ESLint → react-auditor rule mapping, config comparison, CI comparison
- [x] **Doc files:** `ARCHITECTURE.md`, `DEVELOPMENT.md`, `ROADMAP.md`, `RULES.md`, `USAGE.md`, `IMPROVEMENTS.md`, `PLAN.md`

## Phase 19 — Distribution ✅
- [x] **GitHub Action.** `.github/actions/react-auditor/` — composite action with problem matcher; `.github/workflows/audit-pr.yml` — runs on PRs touching JS/TS/JSX/TSX
- [x] **Homebrew tap.** `homebrew/react-auditor.rb` — formula template with placeholders for version + SHA256; auto-published to `chesteralan/homebrew-tap`
- [x] **Docker image.** `docker/Dockerfile` (multi-stage, `debian:bookworm-slim`), `.dockerignore`, `docker-compose.yml`
- [x] **Pre-built binaries.** Release workflow builds 5 targets + uploads artifacts

## Phase 20 — Infrastructure ✅
- [x] **Code coverage.** `.github/workflows/coverage.yml` — `cargo-llvm-cov` generates LCOV, uploaded to Codecov on PR to `main`
- [x] **Dependency dashboard.** `.github/dependabot.yml` — weekly updates for Cargo + GitHub Actions deps
- [x] **Semantic release.** `.github/workflows/release-please.yml` — `release-please` on push to `main`; `release-type: rust`; `extra-files` for `npm/package.json`, `vscode/package.json`

## Phase 21 — CI/CD Refinements ✅
- [x] **All publish workflows trigger on `release: [published]`** — ensures release is fully created before publishing
- [x] **CI trigger narrowed:** CI runs on PRs to `main` + pushes to release-please branch only (no push to `main`)
- [x] **Coverage trigger narrowed:** only PR to `main` (removed push to `main`)
- [x] **Homebrew via `workflow_run`:** triggered by Release workflow completion (not `release: [published]` + sleep hack)
- [x] **Nightly canary deleted** (`.github/workflows/nightly.yml` removed — no longer needed)
- [x] **release-please extra-files:** `npm/package.json` and `vscode/package.json` auto-versioned alongside `Cargo.toml`
- [x] **publish-vscode working-directory:** removed global `defaults: working-directory: vscode` — only applies to vsce publish step
- [x] **Release workflow expanded:** added `aarch64-unknown-linux-gnu` to build matrix

## Phase 21 — Future 🚧

### Short-term
- [ ] Publish v0.3.1 to npm, cargo, Homebrew, VS Code marketplace
- [ ] Add `RELEASE_PLEASE_TOKEN`, `NPM_TOKEN`, `CRATES_TOKEN`, `VSCODE_MARKETPLACE_TOKEN`, `HOMEBREW_TAP_TOKEN` secrets
- [ ] Create `chesteralan/homebrew-tap` repo for formula hosting
- [ ] Publish GitHub Action to marketplace

### Medium-term
- [ ] `prefer-function-components` auto-fix (class → function component conversion)
- [ ] `assert-includes-message` rule (testing category)
- [ ] Config presets (`react-auditor --preset recommended`, `--preset strict`, `--preset nextjs`)
- [ ] Real CI integration test with example Next.js project
- [ ] Fuzz testing: run `cargo +nightly fuzz run fuzz_target_1` in CI (requires nightly)
- [ ] Bundle multi-arch binaries in npm package (currently single-arch per publish)

### Long-term
- [ ] LSP server for editor-agnostic integration
- [ ] Pre-commit hook installer (`react-auditor init`)
- [ ] IntelliJ / JetBrains plugin
- [ ] WASM-based browser playground (current `docs/playground.html` is mock-only)
- [ ] Performance: sub-100µs per 1k LOC target
