use oxc_ast::ast::Program;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoShadow;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-shadow",
    default_severity: Severity::Warning,
    category: "quality",
    description: "No variable shadowing in nested scopes",
};

impl Rule for NoShadow {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, _program: &Program, semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut findings = Vec::new();
        let scoping = semantic.scoping();

        for symbol_id in scoping.symbol_ids() {
            let name = scoping.symbol_name(symbol_id);
            let scope_id = scoping.symbol_scope_id(symbol_id);

            if name.starts_with('_') || name == "arguments" {
                continue;
            }

            let mut ancestors = scoping.scope_ancestors(scope_id);
            ancestors.next();

            for ancestor_scope_id in ancestors {
                if scoping.get_binding(ancestor_scope_id, name).is_some() {
                    let span = scoping.symbol_span(symbol_id);
                    let start = span.start as usize;
                    let line = source_text[..start].lines().count().max(1);
                    let col = start - source_text[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

                    findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: format!("`{name}` shadows a variable in a parent scope"),
                    });
                    break;
                }
            }
        }

        findings
    }
}
