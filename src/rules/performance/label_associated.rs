use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct LabelAssociated;

const RULE_META: RuleMeta = RuleMeta {
    id: "label-associated",
    default_severity: Severity::Warning,
    category: "accessibility",
    description: "Form inputs should have associated labels",
};

impl Rule for LabelAssociated {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = LabelCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct LabelCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for LabelCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let is_input = matches!(&el.name, oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "input" || id.name.as_str() == "select" || id.name.as_str() == "textarea");
        if !is_input {
            return;
        }
        let has_aria_label = el.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr {
                matches!(&a.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "aria-label" || id.name.as_str() == "aria-labelledby")
            } else {
                false
            }
        });
        let has_id = el.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr {
                matches!(&a.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "id")
            } else {
                false
            }
        });
        if !has_aria_label && !has_id {
            let start = el.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Form input is missing an associated `<label>` or `aria-label`".to_string(),
            });
        }
    }
}
