use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::rules::{RuleRegistry, Severity};

pub fn generate_docs() {
    let registry = RuleRegistry::new();
    let ids = registry.get_rule_ids();

    let mut categories: BTreeMap<&str, Vec<(&str, &str, &str)>> = BTreeMap::new();

    for id in &ids {
        if let Some(rule) = registry.get_rule(id) {
            let meta = rule.meta();
            categories.entry(meta.category).or_default().push((
                meta.id,
                meta.description,
                severity_str(&meta.default_severity),
            ));
        }
    }

    let docs_dir = Path::new("docs/rules");
    fs::create_dir_all(docs_dir).expect("failed to create docs/rules");

    for id in &ids {
        if let Some(rule) = registry.get_rule(id) {
            let meta = rule.meta();
            let filename = format!("{}.md", meta.id);
            let path = docs_dir.join(&filename);
            let content = format!(
                concat!(
                    "# {}\n\n",
                    "- **Category:** {}\n",
                    "- **Default severity:** {}\n",
                    "- **Auto-fix:** {}\n\n",
                    "{}\n",
                ),
                meta.id,
                meta.category,
                severity_str(&meta.default_severity),
                if rule.has_fix() { "Yes" } else { "No" },
                meta.description,
            );
            fs::write(&path, content).expect("failed to write rule doc");
            println!("wrote {}", path.display());
        }
    }

    let mut index = String::from("# Rules\n\nAll rules organized by category.\n\n");
    for (category, rules) in &categories {
        index.push_str(&format!("## {}\n\n", capitalize(category)));
        index.push_str("| Rule | Default | Description |\n|------|---------|-------------|\n");
        for (id, desc, sev) in rules {
            index.push_str(&format!("| [{}]({}.md) | {} | {} |\n", id, id, sev, desc));
        }
        index.push('\n');
    }

    let index_path = docs_dir.join("README.md");
    fs::write(&index_path, index).expect("failed to write index");
    println!("wrote {}", index_path.display());
}

fn severity_str(s: &Severity) -> &str {
    match s {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Off => "off",
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}
