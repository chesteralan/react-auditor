# Improvements

## CLI & UX

- **Wire `--rules` flag into Scanner.** Flag is declared in `Cli` but ignored — `Scanner` doesn't filter by category. Just need to pass it through `run_rules`.
- **Add `--ignore` / `--exclude` patterns.** Useful for skipping `node_modules`, `dist/`, `build/`, etc. without relying solely on the source glob.
- **Add `--fail-on` severity level.** e.g. `--fail-on error` exits 1 only if errors exist (ignore warnings).
- **Colored output for `stylish` formatter.** Currently plain text. Use `termcolor` for red/green/yellow severity indicators.
- **Show rule category in output.** e.g. `[react/no-missing-key]` instead of just `[no-missing-key]`.
- **Spinner/progress bar during scan.** Current `"[i/N] path"` stderr output is functional but could be prettier with `indicatif`.

## Rules

- **React: `no-direct-mutation`.** Detect `this.props.x = y` or direct `state` mutation outside `setState`.
- **React: `jsx-no-duplicate-props`.** Flag repeated props like `<div id="a" id="b" />`.
- **React: `no-array-index-key`.** Warn when `<li key={index}>` is used (separate from existing `no-index-key` which covers broader cases).
- **React: `no-ref-in-component-name`.** Component names shouldn't contain "Ref" unless it's a ref-forwarding wrapper.
- **TypeScript: `no-explicit-any` (stricter).** Current `no-any` only flags `any` type annotations — could also catch `as any`, `as unknown as any`.
- **Security: `no-unsafe-iframe`.** Warn on `<iframe>` without `sandbox` or `title` attributes.
- **Next.js: `no-sync-script`.** Flag synchronous `<Script>` without `strategy="afterInteractive"`.
- **Performance: `no-large-libraries`.** Warn on importing heavy libraries (moment, lodash entire) when lighter alternatives exist.
- **Accessibility: `a-has-content`.** Warn on `<a>` or `<button>` with no text content or `aria-label`.
- **Accessibility: `no-ambiguous-labels`.** Warn on duplicate or ambiguous label text.
- **Auto-fix for more rules.** Currently only `no-var` and `no-inline-styles` support `--fix`. Add to `no-empty-blocks`, `no-console` (strip), `prefer-fragments`.

## Scanner & Engine

- **Incremental / cached scanning.** Only re-scan changed files since last run. Store file hashes in `.raudit-cache.json`.
- **Parallel file scanning.** Use `rayon` or `std::thread` to scan files concurrently. oxc parsing is fast, but I/O + rule overhead scales.
- **Multi-root workspace support.** Allow scanning multiple directories in a single invocation.
- **Watch mode.** `react-auditor --watch` — re-scan on file changes using `notify` crate.
- **Configurable rule defaults per file type.** e.g. disable TS rules for `.jsx` files.

## Testing

- **Snapshot testing for formatters.** Run fixtures through `stylish`/`json`/`compact` formatters and compare against snapshots.
- **Property-based testing.** Use `proptest` or `arbitrary` to generate random AST fragments and verify rules don't panic.
- **Integration test with real-world projects.** Clone a small React project and scan it end-to-end.
- **Benchmark suite.** Use `criterion` to track scan time per 1000 LOC, catch regressions.
- **Fuzz the parser.** Feed random bytes to the parser to ensure graceful error handling.

## VS Code Extension

- **Error list / problems panel grouping.** Group diagnostics by rule ID so users can see all violations of the same rule.
- **Quick-fix code actions.** Suggest `--fix` for auto-fixable rules directly from the editor.
- **Configuration UI.** Add a settings page for `.rauditrc.toml` generation instead of only JSON/Toml file editing.
- **Progress notification.** Show a progress bar when running workspace scan (50 files).
- **Decorations.** Inline gutter markers for violations, not just problem panel entries.

## Documentation

- **Rule documentation generator.** Auto-generate per-rule markdown from `RuleMeta` (including examples) — like ESLint's rule docs.
- **Website / playground.** A simple HTML page demonstrating output formats and rule examples.
- **Migration guides.** ESLint → react-auditor config mapping for common rules.

## Distribution

- **GitHub Action.** A `react-auditor-action` that runs on PRs and posts annotations via `problem matchers`.
- **Homebrew tap.** `brew install react-auditor` for macOS users.
- **Docker image.** `docker run react-auditor scan src/`.
- **Pre-built binaries for Windows.** CI currently builds Linux + macOS (aarch64). Add Windows via `cross`.

## Infrastructure

- **Automated canary releases.** Nightly builds from `main` for early adopters.
- **Code coverage.** Track line/branch coverage in CI with `tarpaulin` or `cargo-llvm-cov`.
- **Dependency dashboard.** Use Dependabot or Renovate to keep oxc and other deps current.
- **Semantic release.** Automate version bumps, changelog generation, and tag creation from commit messages.
