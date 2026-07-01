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
- [x] `no-missing-deps`
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
- [x] Tag a `v0.1.0` release to trigger npm + VS Code publishing

## Phase 12 — Next.js Rules ✅
- [x] `no-img-element` (use `<Image />` instead of `<img>`)
- [x] `no-script-tag-in-head` (use `<Script />` instead of `<script>` inside `<Head>`)
- [x] `no-page-link` (use `<Link>` instead of `<a>` for internal navigation)
- [x] `no-head-element` (use `<Head />` instead of `<head>`)

## Phase 13 — CLI & UX ✅
- [x] **Wire `--rules` flag into Scanner.** Flag is declared in `Cli` but ignored — `Scanner` doesn't filter by category. Just need to pass it through `run_rules`.
- [x] **Add `--ignore` / `--exclude` patterns.** Skip `node_modules`, `dist/`, `build/`, etc. without relying solely on the source glob.
- [x] **Add `--fail-on` severity level.** e.g. `--fail-on error` exits 1 only if errors exist.
- [x] **Colored output for `stylish` formatter.** Use `termcolor` for red/green/yellow severity indicators.
- [x] **Show rule category in output.** e.g. `[react/no-missing-key]` instead of just `[no-missing-key]`.
- [x] **Spinner/progress bar during scan.** Prettier progress with `indicatif`.

## Phase 14 — New Rules ✅
- [x] **React: `no-direct-mutation`.** Detect `this.props.x = y` or direct `state` mutation outside `setState`.
- [x] **React: `jsx-no-duplicate-props`.** Flag repeated props like `<div id="a" id="b" />`.
- [x] **React: `no-array-index-key`.** Already covered by `no-index-key` rule (line 45).
- [x] **React: `no-ref-in-component-name`.** Component names shouldn't contain "Ref".
- [x] **TypeScript: `no-explicit-any` (stricter).** Catch `as any`, `<any>`, and type annotations with `any`.
- [x] **Security: `no-unsafe-iframe`.** Warn on `<iframe>` without `sandbox` or `title`.
- [x] **Next.js: `no-sync-script`.** Flag synchronous `<Script>` without `strategy="afterInteractive"`.
- [x] **Performance: `no-large-libraries`.** Warn on importing heavy libraries (moment, lodash) when lighter alternatives exist.
- [x] **Accessibility: `a-has-content`.** Warn on `<a>` or `<button>` with no text content or `aria-label`.
- [x] **Accessibility: `no-ambiguous-labels`.** Warn on duplicate or ambiguous label text.
- [x] **Auto-fix for more rules.** Extend `--fix` to `no-empty-blocks`, `no-console` (strip).

## Phase 15 — Scanner & Engine
- [x] **Parallel file scanning.** Use `rayon` for concurrent file processing.
- [x] **Incremental / cached scanning.** Only re-scan changed files via `.raudit-cache.json` file mtime hashes.
- [x] **Watch mode.** `react-auditor -W` via `notify` crate, with 200ms debounce.
- [x] **Multi-root workspace support.** Expand workspace globs from config (`workspaces` field) into scan roots.
- [x] **Configurable rule defaults per file type.** `file_types` config field maps extension → rule overrides (e.g. disable TS rules for `.jsx`).

## Phase 16 — Testing
- [x] **Snapshot testing for formatters.** Compare fixture output against snapshots for `stylish`/`json`/`compact`.
- [x] **Property-based testing.** Use `proptest` to generate random AST fragments and verify rules don't panic.
- [x] **Integration test with real-world projects.** Walk a realistic multi-file project under `tests/real-project/` and scan end-to-end.
- [x] **Benchmark suite.** Use `criterion` to track scan time per 1000 LOC (bench results: ~138µs for 1k LOC).
- [x] **Fuzz the parser.** Feed random byte sequences, edge cases, and malformed input to ensure graceful error handling.

## Phase 17 — VS Code Extension
- [x] **Error list / problems panel grouping.** Diagnostics grouped by `ruleId` via `Diagnostic.code`, plus `relatedInformation` for category context.
- [x] **Quick-fix code actions.** `CodeActionProvider` provides "Fix with react-auditor" (runs `--fix`) and "Disable rule" options from the Problems panel.
- [x] **Configuration UI.** Webview panel (`react-auditor.configure`) with dropdowns for common rules — generates `.rauditrc.toml` on save.
- [x] **Progress notification.** `withProgress()` API for workspace scans with cancellable progress bar.
- [x] **Decorations.** Gutter highlight and overview ruler markers for error/warning lines.

## Phase 18 — Documentation
- [x] **Rule documentation generator.** `cargo run --bin docgen` generates per-rule markdown in `docs/rules/` from `RuleMeta`, including category, severity, and auto-fix status.
- [x] **Website / playground.** `docs/playground.html` — client-side HTML playground with built-in examples (basic, hooks, security, TypeScript, Next.js) and mock detection for offline demo.
- [x] **Migration guides.** `docs/migration-from-eslint.md` — comprehensive ESLint → react-auditor rule mapping, config comparison, CI comparison, and performance benchmarks.

## Phase 19 — Distribution
- [x] **GitHub Action.** `.github/actions/react-auditor/` — composite action with problem matcher; `.github/workflows/audit-pr.yml` — runs on PRs touching JS/TS/JSX/TSX.
- [x] **Homebrew tap.** `homebrew/react-auditor.rb` formula for macOS (ARM + Intel) and Linux. Publish to `chesteralan/homebrew-tap`.
- [x] **Docker image.** `docker/Dockerfile` (multi-stage, `debian:bookworm-slim`), `.dockerignore`, `docker-compose.yml`.
- [x] **Pre-built binaries.** `.github/workflows/release.yml` — builds Linux x86_64, macOS x86_64 + ARM, Windows x86_64 via `actions-rs/cargo`. Archives uploaded as release artifacts on tag push.

## Phase 20 — Infrastructure
- [x] **Automated canary releases.** `.github/workflows/nightly.yml` — builds + tests daily at 06:00 UTC, uploads binary as artifact. Manual dispatch also supported.
- [x] **Code coverage.** `.github/workflows/coverage.yml` — `cargo-llvm-cov` generates LCOV report, uploaded to Codecov on push/PR to `main`.
- [x] **Dependency dashboard.** `.github/dependabot.yml` — weekly updates for Cargo + GitHub Actions dependencies, labeled `dependencies`/`rust`/`ci`.
- [x] **Semantic release.** `.github/workflows/release-please.yml` — `release-please` action on push to `main` using conventional commits; `release-type: rust` to auto-bump `Cargo.toml` version and generate changelog.
