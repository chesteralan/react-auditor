use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Preset {
    Recommended,
    Strict,
    Nextjs,
    All,
}

impl std::str::FromStr for Preset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "recommended" => Ok(Preset::Recommended),
            "strict" => Ok(Preset::Strict),
            "nextjs" => Ok(Preset::Nextjs),
            "all" => Ok(Preset::All),
            _ => Err(format!(
                "Unknown preset: {s}. Available: recommended, strict, nextjs, all"
            )),
        }
    }
}

impl Preset {
    pub fn severity_overrides(&self) -> HashMap<String, String> {
        match self {
            Preset::Recommended => HashMap::new(),
            Preset::Strict => {
                let mut map = HashMap::new();
                map.insert("no-console".to_string(), "error".to_string());
                map.insert("no-any".to_string(), "error".to_string());
                map.insert("no-eval".to_string(), "error".to_string());
                map.insert("no-non-null-assertion".to_string(), "error".to_string());
                map.insert("no-type-assertion".to_string(), "error".to_string());
                map.insert("no-unused-vars".to_string(), "error".to_string());
                map.insert("no-shadow".to_string(), "error".to_string());
                map.insert(
                    "prefer-function-components".to_string(),
                    "error".to_string(),
                );
                map.insert("explicit-return-type".to_string(), "error".to_string());
                map.insert(
                    "no-dangerously-set-innerhtml".to_string(),
                    "error".to_string(),
                );
                map.insert("no-hardcoded-secrets".to_string(), "error".to_string());
                map.insert("no-unsanitized-input".to_string(), "error".to_string());
                map
            }
            Preset::Nextjs => {
                let mut map = HashMap::new();
                map.insert("no-console".to_string(), "warn".to_string());
                map.insert("no-img-element".to_string(), "error".to_string());
                map.insert("no-script-tag-in-head".to_string(), "error".to_string());
                map.insert("no-page-link".to_string(), "error".to_string());
                map.insert("no-head-element".to_string(), "error".to_string());
                map.insert("no-sync-script".to_string(), "error".to_string());
                map.insert("no-missing-key".to_string(), "error".to_string());
                map.insert("no-inline-styles".to_string(), "warn".to_string());
                map
            }
            Preset::All => {
                let mut map = HashMap::new();
                map.insert("complexity".to_string(), "error".to_string());
                map.insert("max-params".to_string(), "error".to_string());
                map.insert("no-long-functions".to_string(), "error".to_string());
                map.insert("no-deep-nesting".to_string(), "error".to_string());
                map.insert("no-magic-numbers".to_string(), "warn".to_string());
                map
            }
        }
    }

    pub fn category_filter(&self) -> Option<Vec<String>> {
        match self {
            Preset::Nextjs => Some(vec![
                "react".to_string(),
                "nextjs".to_string(),
                "performance".to_string(),
                "accessibility".to_string(),
                "quality".to_string(),
            ]),
            _ => None,
        }
    }
}
