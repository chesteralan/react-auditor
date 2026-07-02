use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoAutofocus;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-autofocus",
    default_severity: Severity::Warning,
    category: "accessibility",
    description: "Avoid autoFocus attribute; it can cause usability issues",
};

impl Rule for NoAutofocus {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = AutofocusCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct AutofocusCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for AutofocusCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr
                && let oxc_ast::ast::JSXAttributeName::Identifier(id) = &a.name
                && id.name.as_str() == "autoFocus"
            {
                let start = a.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col =
                    start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: "Avoid autoFocus attribute for accessibility".to_string(),
                });
            }
        }
    }
}
