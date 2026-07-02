# react-auditor

A blazing-fast Rust CLI to scan JS/TS/React code for best practices, quality, security, performance, Next.js, and accessibility issues.

Powered by [oxc](https://oxc.rs/) — parallel scanning at ~138µs per 1k LOC. 67 rules across 7 categories.

This npm package is a thin wrapper that requires the `react-auditor` binary on PATH. Install it first:

```bash
cargo install react-auditor
# or: brew install react-auditor
```

## Usage

```bash
npx react-auditor src/
```

Or install globally:

```bash
npm install -g react-auditor
react-auditor src/
```

## Documentation

See the [GitHub README](https://github.com/chesteralan/react-auditor#readme) for full documentation, configuration, and rule reference.
