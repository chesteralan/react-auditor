# Usage

## Installation

### Option 1: npm (recommended for Node projects)

```bash
npm install --save-dev react-auditor
```

The npm package downloads the correct pre-built binary for your platform.

### Option 2: cargo

```bash
cargo install react-auditor
```

### Option 3: GitHub Releases

Download the binary for your platform from the latest release.

## CLI

```bash
react-auditor [files...] [options]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `files` | One or more file paths or glob patterns. If omitted, scans `src/**/*.{js,jsx,ts,tsx}` |

### Options

| Flag | Alias | Description |
|------|-------|-------------|
| `--config <path>` | `-c` | Path to config file |
| `--rules <categories>` | `-r` | Comma-separated rule categories: `quality`, `react`, `typescript`, `security`, `performance`, `accessibility`, `testing` |
| `--log <path>` | `-l` | Path to output log file |
| `--format <format>` | `-f` | Output format: `stylish` (default), `json`, `compact` |
| `--max-warnings <n>` | `-w` | Number of warnings before exiting with code 1 |
| `--quiet` | `-q` | Only output errors, no warnings |
| `--fix` | | Auto-fix where supported |
| `--help` | `-h` | Show help |
| `--version` | `-v` | Show version |

### Examples

```bash
# Scan all files in src
react-auditor

# Scan specific files
react-auditor src/components/Button.tsx src/hooks/useAuth.ts

# Scan with only React and TypeScript rules
react-auditor --rules react,typescript

# Output to terminal and log file
react-auditor --log audit-results.json

# JSON output
react-auditor --format json

# Fail CI if more than 10 warnings
react-auditor --max-warnings 10
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

## Configuration File

`.rauditrc.toml`:

```toml
[rules]
no-console = "error"
no-any = "warn"
no-magic-numbers = "off"

log = "audit-results.json"
format = "stylish"
max_warnings = 10
```

`.rauditrc.json` also supported:

```json
{
  "rules": {
    "no-console": "error",
    "no-any": "warn",
    "no-magic-numbers": "off"
  },
  "log": "audit-results.json",
  "format": "stylish",
  "maxWarnings": 10
}
```

Or in `package.json`:

```json
{
  "reactAuditor": {
    "rules": {
      "no-console": "error"
    }
  }
}
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | No errors (warnings may exist if below `maxWarnings`) |
| 1 | Errors found or warnings exceed `maxWarnings` |
