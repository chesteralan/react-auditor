use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub rules: HashMap<String, String>,

    #[serde(default)]
    pub log: Option<String>,

    #[serde(default = "default_format")]
    #[allow(dead_code)]
    pub format: String,

    #[serde(default)]
    pub max_warnings: Option<u32>,

    /// Multi-root workspace patterns (monorepo support).
    /// e.g. ["packages/*", "apps/*"]
    #[serde(default)]
    pub workspaces: Vec<String>,

    /// Per-file-type rule overrides.
    /// Key is file extension (e.g. "jsx"), value is rule_id → severity.
    #[serde(default)]
    pub file_types: HashMap<String, HashMap<String, String>>,
}

fn default_format() -> String {
    "stylish".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rules: HashMap::new(),
            log: None,
            format: "stylish".to_string(),
            max_warnings: None,
            workspaces: Vec::new(),
            file_types: HashMap::new(),
        }
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> anyhow::Result<Self> {
        let mut config = Self::default();

        if let Some(config_path) = path {
            if !config_path.exists() {
                anyhow::bail!("Config file not found: {}", config_path.display());
            }

            let content = std::fs::read_to_string(config_path)?;
            let file_name = config_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();

            config = if file_name == "package.json" {
                parse_package_json_config(&content)?
            } else {
                match config_path.extension().and_then(|e| e.to_str()) {
                    Some("toml") => parse_toml_config(&content)?,
                    Some("json") | Some("jsonc") => parse_json_config(&content)?,
                    _ => parse_json_config(&content)?,
                }
            };
        } else {
            // Try standard config file paths
            let candidates = [".rauditrc.toml", ".rauditrc.json", ".rauditrc"];

            for candidate in &candidates {
                let p = Path::new(candidate);
                if p.exists() {
                    config = Self::load(Some(p))?;
                    break;
                }
            }

            // Try package.json
            let pkg_path = Path::new("package.json");
            if pkg_path.exists()
                && let Ok(content) = std::fs::read_to_string(pkg_path)
                && let Ok(pkg_config) = parse_package_json_config(&content)
            {
                merge_config(&mut config, pkg_config);
            }
        }

        config.file_types = normalize_file_type_overrides(config.file_types);

        Ok(config)
    }
}

#[derive(Debug, Default, Deserialize)]
struct ConfigWithOptionalRules {
    #[serde(default)]
    rules: HashMap<String, String>,
    #[serde(default)]
    log: Option<String>,
    #[serde(default = "default_format")]
    format: String,
    #[serde(default)]
    max_warnings: Option<u32>,
    #[serde(default)]
    workspaces: Vec<String>,
    #[serde(default)]
    file_types: HashMap<String, HashMap<String, String>>,
}

fn parse_toml_config(content: &str) -> anyhow::Result<Config> {
    let parsed: ConfigWithOptionalRules = toml::from_str(content)?;
    let mut config = Config {
        rules: parsed.rules,
        log: parsed.log,
        format: parsed.format,
        max_warnings: parsed.max_warnings,
        workspaces: parsed.workspaces,
        file_types: normalize_file_type_overrides(parsed.file_types),
    };

    let value: toml::Value = toml::from_str(content)?;
    if let Some(table) = value.as_table() {
        for (key, val) in table {
            if is_known_config_field(key) {
                continue;
            }
            if let Some(severity) = val.as_str() {
                config.rules.insert(key.clone(), severity.to_string());
            }
        }
    }

    Ok(config)
}

fn parse_json_config(content: &str) -> anyhow::Result<Config> {
    let parsed: ConfigWithOptionalRules = serde_json::from_str(content)?;
    let mut config = Config {
        rules: parsed.rules,
        log: parsed.log,
        format: parsed.format,
        max_warnings: parsed.max_warnings,
        workspaces: parsed.workspaces,
        file_types: normalize_file_type_overrides(parsed.file_types),
    };

    let value: serde_json::Value = serde_json::from_str(content)?;
    if let Some(map) = value.as_object() {
        for (key, val) in map {
            if is_known_config_field(key) {
                continue;
            }
            if let Some(severity) = val.as_str() {
                config.rules.insert(key.clone(), severity.to_string());
            }
        }
    }

    Ok(config)
}

fn parse_package_json_config(content: &str) -> anyhow::Result<Config> {
    let pkg: PackageJson = serde_json::from_str(content)?;
    let mut config = Config::default();

    if let Some(auditor) = pkg.react_auditor {
        config.rules = auditor.rules;
        config.log = auditor.log;
        config.format = auditor.format;
        config.max_warnings = auditor.max_warnings;
        config.workspaces = auditor.workspaces;
        config.file_types = normalize_file_type_overrides(auditor.file_types);
    }

    Ok(config)
}

fn merge_config(base: &mut Config, overlay: Config) {
    base.rules.extend(overlay.rules);
    if base.log.is_none() {
        base.log = overlay.log;
    }
    if base.format == default_format() && overlay.format != default_format() {
        base.format = overlay.format;
    }
    if base.max_warnings.is_none() {
        base.max_warnings = overlay.max_warnings;
    }
    if base.workspaces.is_empty() && !overlay.workspaces.is_empty() {
        base.workspaces = overlay.workspaces;
    }
    for (ext, overrides) in overlay.file_types {
        base.file_types.entry(ext).or_default().extend(overrides);
    }
}

fn is_known_config_field(key: &str) -> bool {
    matches!(
        key,
        "rules" | "log" | "format" | "max_warnings" | "workspaces" | "file_types"
    )
}

fn normalize_file_type_overrides(
    input: HashMap<String, HashMap<String, String>>,
) -> HashMap<String, HashMap<String, String>> {
    let mut normalized = HashMap::new();
    for (ext, overrides) in input {
        let key = ext.trim().trim_start_matches('.').to_ascii_lowercase();
        if key.is_empty() {
            continue;
        }
        normalized
            .entry(key)
            .or_insert_with(HashMap::new)
            .extend(overrides);
    }
    normalized
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    #[serde(rename = "reactAuditor")]
    react_auditor: Option<PackageConfig>,
}

#[derive(Debug, Deserialize)]
struct PackageConfig {
    #[serde(default)]
    rules: HashMap<String, String>,
    #[serde(default)]
    log: Option<String>,
    #[serde(default = "default_format")]
    format: String,
    #[serde(default)]
    max_warnings: Option<u32>,
    #[serde(default)]
    workspaces: Vec<String>,
    #[serde(default)]
    file_types: HashMap<String, HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::Config;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_test_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        let pid = std::process::id();
        let dir = std::env::temp_dir().join(format!("react_auditor_config_test_{pid}_{nanos}"));
        fs::create_dir_all(&dir).expect("failed to create test dir");
        dir
    }

    #[test]
    fn supports_top_level_rules_and_rules_table_in_toml() {
        let dir = unique_test_dir();
        let path = dir.join(".rauditrc.toml");
        let content = r#"
"no-console" = "warning"

[rules]
"no-var" = "error"
"#;
        fs::write(&path, content).expect("failed to write config");

        let cfg = Config::load(Some(&path)).expect("config should parse");
        assert_eq!(
            cfg.rules.get("no-console").map(String::as_str),
            Some("warning")
        );
        assert_eq!(cfg.rules.get("no-var").map(String::as_str), Some("error"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn normalizes_file_type_override_keys_with_or_without_dot() {
        let dir = unique_test_dir();
        let path = dir.join(".rauditrc.toml");
        let content = r#"
[file_types]
".jsx" = { "no-explicit-any" = "off" }
"jsx" = { "no-console" = "warning" }
"TSX" = { "no-var" = "error" }
"#;
        fs::write(&path, content).expect("failed to write config");

        let cfg = Config::load(Some(&path)).expect("config should parse");
        let jsx = cfg.file_types.get("jsx").expect("expected jsx overrides");
        assert_eq!(jsx.get("no-explicit-any").map(String::as_str), Some("off"));
        assert_eq!(jsx.get("no-console").map(String::as_str), Some("warning"));
        let tsx = cfg.file_types.get("tsx").expect("expected tsx overrides");
        assert_eq!(tsx.get("no-var").map(String::as_str), Some("error"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn supports_full_package_json_react_auditor_config() {
        let dir = unique_test_dir();
        let path = dir.join("package.json");
        let content = r#"
{
  "name": "demo",
  "reactAuditor": {
    "rules": { "no-console": "off" },
    "log": "audit.json",
    "format": "compact",
    "max_warnings": 7,
    "workspaces": ["apps/*"],
    "file_types": {
      ".jsx": { "no-explicit-any": "off" }
    }
  }
}
"#;
        fs::write(&path, content).expect("failed to write package.json");

        let cfg = Config::load(Some(&path)).expect("package config should parse");
        assert_eq!(cfg.rules.get("no-console").map(String::as_str), Some("off"));
        assert_eq!(cfg.log.as_deref(), Some("audit.json"));
        assert_eq!(cfg.format, "compact");
        assert_eq!(cfg.max_warnings, Some(7));
        assert_eq!(cfg.workspaces, vec!["apps/*".to_string()]);
        let jsx = cfg.file_types.get("jsx").expect("expected jsx overrides");
        assert_eq!(jsx.get("no-explicit-any").map(String::as_str), Some("off"));

        let _ = fs::remove_dir_all(&dir);
    }
}
