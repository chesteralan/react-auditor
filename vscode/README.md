# React Auditor

Real-time linting with `react-auditor` — best practices, quality, security, and accessibility for React codebases.

Powered by [oxc](https://oxc.rs) for blazing-fast performance.

## Features

- **67 rules** across 7 categories: quality, react, typescript, security, nextjs, accessibility, performance
- Runs on save (configurable to run on change or open)
- Status bar indicator showing issue count
- Per-file diagnostics with rule ID and message
- Supports JavaScript, JSX, TypeScript, and TSX

## Commands

| Command | Title |
|---------|-------|
| `react-auditor.run` | Run on current file |
| `react-auditor.runWorkspace` | Run on workspace (first 50 files) |
| `react-auditor.clear` | Clear all diagnostics |

## Requirements

- `react-auditor` binary installed via `cargo install react-auditor`

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `reactAuditor.binaryPath` | `react-auditor` | Path to the binary |
| `reactAuditor.runOnSave` | `true` | Run on file save |
| `reactAuditor.runOnChange` | `false` | Run on file change (debounced) |
| `reactAuditor.runOnOpen` | `false` | Run when file opens |

## Configuration

Create `.rauditrc.toml`, `.rauditrc.json`, or add `reactAuditor` to `package.json` to configure rule severity and categories.

## License

MIT
