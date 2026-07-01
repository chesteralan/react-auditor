use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use react_auditor::rules::RuleRegistry;

fn main() {
    let registry = RuleRegistry::new();
    let ids = registry.get_rule_ids();

    // Collect rules grouped by category
    let mut categories: BTreeMap<&str, Vec<(&str, &str, &str)>> = BTreeMap::new();

    for id in &ids {
        if let Some(rule) = registry.get_rule(id) {
            let meta = rule.meta();
            categories.entry(meta.category).or_default().push((
                meta.id,
                meta.description,
                meta.default_severity_str(),
            ));
        }
    }

    let docs_dir = Path::new("docs/rules");
    fs::create_dir_all(docs_dir).expect("failed to create docs/rules");

    // Generate per-rule pages
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
                meta.default_severity_str(),
                if rule.has_fix() { "Yes" } else { "No" },
                meta.description,
            );
            fs::write(&path, content).expect("failed to write rule doc");
            println!("wrote {}", path.display());
        }
    }

    // Generate index
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

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

trait DefaultSeverityStr {
    fn default_severity_str(&self) -> &str;
}

impl DefaultSeverityStr for react_auditor::rules::RuleMeta {
    fn default_severity_str(&self) -> &str {
        match self.default_severity {
            react_auditor::rules::Severity::Error => "error",
            react_auditor::rules::Severity::Warning => "warning",
            react_auditor::rules::Severity::Off => "off",
        }
    }
}
