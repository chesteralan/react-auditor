use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct ButtonHasType;

const RULE_META: RuleMeta = RuleMeta {
    id: "button-has-type",
    default_severity: Severity::Error,
    category: "accessibility",
    description: "Buttons should have explicit `type` attribute",
};

impl Rule for ButtonHasType {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ButtonTypeCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ButtonTypeCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ButtonTypeCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let is_button = matches!(&el.name, oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "button");
        if !is_button {
            return;
        }
        let has_type = el.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr {
                matches!(&a.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "type")
            } else {
                false
            }
        });
        if !has_type {
            let start = el.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Missing `type` attribute on `<button>` — defaults to `submit`"
                    .to_string(),
            });
        }
    }
}
