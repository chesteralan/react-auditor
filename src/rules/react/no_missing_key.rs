use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoMissingKey;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-missing-key",
    default_severity: Severity::Error,
    category: "react",
    description: "List items should have a `key` prop",
};

impl Rule for NoMissingKey {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = MissingKeyCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct MissingKeyCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for MissingKeyCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let has_key = el.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr {
                matches!(&a.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "key")
            } else {
                false
            }
        });

        if !has_key {
            let start = el.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

            let name_str = match &el.name {
                oxc_ast::ast::JSXElementName::Identifier(id) => Some(id.name.as_str()),
                oxc_ast::ast::JSXElementName::IdentifierReference(id) => Some(id.name.as_str()),
                _ => None,
            };

            if let Some(name) = name_str
                && name.chars().next().is_some_and(|c| c.is_uppercase()) {
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: format!("Component `<{name}>` is missing a `key` prop"),
                    });
                }
        }
    }
}
