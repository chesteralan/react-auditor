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
        }
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> anyhow::Result<Self> {
        let mut config = Self::default();

        if let Some(config_path) = path {
            if config_path.exists() {
                let content = std::fs::read_to_string(config_path)?;
                match config_path.extension().and_then(|e| e.to_str()) {
                    Some("toml") => config = toml::from_str(&content)?,
                    Some("json") | Some("jsonc") => config = serde_json::from_str(&content)?,
                    _ => config = serde_json::from_str(&content)?,
                }
            }
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
                && let Ok(pkg) = serde_json::from_str::<PackageJson>(&content)
                && let Some(auditor) = pkg.react_auditor
            {
                config.rules.extend(auditor.rules);
                config.log = config.log.or(auditor.log);
                config.max_warnings = config.max_warnings.or(auditor.max_warnings);
            }
        }

        Ok(config)
    }
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
    #[serde(default)]
    max_warnings: Option<u32>,
}
