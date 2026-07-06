use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "react-auditor",
    version,
    about = "A blazing-fast Rust CLI to scan JS/TS/React code for best practices, quality, and security issues",
    long_about = "react-auditor — Scan JS/TS/React Code for Best Practices

A blazing-fast Rust CLI powered by oxc to lint your React codebase for
quality, correctness, security, performance, and accessibility issues.

Categories (71 rules total):
  quality        Code quality & clean code (13 rules)
  react          React best practices & hooks (19 rules)
  typescript     TypeScript strictness (9 rules)
  security       Security vulnerabilities (7 rules)
  nextjs         Next.js best practices (5 rules)
  performance    Performance anti-patterns (5 rules)
  accessibility  Accessibility violations (11 rules)
  testing        Test quality & correctness (2 rules)

Presets:
  recommended    Default severity for all rules
  strict         Bump many rules to Error
  nextjs         Focus on Next.js + React rules
  all            Maximum severity for all rules

Integrated with lint-staged and husky for pre-commit checks.

Examples:
  react-auditor                              Scan src/**/*.{js,jsx,ts,tsx}
  react-auditor src/ --format json            Scan src/ output as JSON
  react-auditor --rules react,typescript      Only React & TS rules
  react-auditor --preset strict               Use strict preset
  react-auditor --ignore node_modules,dist    Skip node_modules and dist
  react-auditor --log audit.json              Write JSON log file
  react-auditor --max-warnings 10             Fail on >10 warnings
  react-auditor --fail-on warning             Fail on any violation
  react-auditor --fix                         Auto-fix where supported
  react-auditor init                          Install pre-commit hook
  react-auditor --generate-config > .rauditrc.toml   Create default config
)]
pub struct Cli {
    /// File paths or glob patterns to scan (default: src/**/*.{js,jsx,ts,tsx})
    pub files: Vec<String>,

    /// Path to config file (.rauditrc.toml, .rauditrc.json, package.json)
    #[arg(short = 'c', long = "config")]
    pub config: Option<String>,

    /// Comma-separated rule categories to enable: quality, react, typescript, security, nextjs, performance, accessibility, testing
    #[arg(short = 'r', long = "rules")]
    pub rules: Option<String>,

    /// Configuration preset: recommended (default), strict, nextjs, all
    #[arg(long = "preset", default_value = "recommended")]
    pub preset: String,

    /// Fail on severity level: error, warning (exit code 1 if any violations at or above this level)
    #[arg(long = "fail-on", default_value = "error")]
    pub fail_on: String,

    /// Path to output JSON log file
    #[arg(short = 'l', long = "log")]
    pub log: Option<String>,

    /// Output format: stylish (default), json, compact
    #[arg(short = 'f', long = "format", default_value = "stylish")]
    pub format: String,

    /// Max warnings before exiting with code 1
    #[arg(short = 'w', long = "max-warnings")]
    pub max_warnings: Option<u32>,

    /// Only output errors (suppress warnings)
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Comma-separated glob patterns to ignore (e.g. node_modules,dist,build)
    #[arg(long = "ignore", default_value = "")]
    pub ignore: String,

    /// Auto-fix violations where supported (currently: no-var, no-console, no-empty-blocks, prefer-fragments, prefer-function-components)
    #[arg(long = "fix")]
    pub fix: bool,

    /// Disable incremental caching (re-scan all files)
    #[arg(long = "no-cache")]
    pub no_cache: bool,

    /// Watch mode: re-scan on file changes
    #[arg(short = 'W', long = "watch")]
    pub watch: bool,

    /// Generate rule documentation in docs/rules/
    #[arg(long = "docs")]
    pub docs: bool,

    /// Generate a default config file (.rauditrc.toml) to stdout
    #[arg(long = "generate-config")]
    pub generate_config: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install a pre-commit git hook that runs react-auditor on staged files
    Init,
}
