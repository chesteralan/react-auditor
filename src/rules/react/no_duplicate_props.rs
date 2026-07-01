use std::collections::HashSet;

use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoDuplicateProps;

const RULE_META: RuleMeta = RuleMeta {
    id: "jsx-no-duplicate-props",
    default_severity: Severity::Error,
    category: "react",
    description: "Duplicate props are not allowed in JSX",
};

impl Rule for NoDuplicateProps {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = DuplicatePropsCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct DuplicatePropsCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for DuplicatePropsCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let mut seen: HashSet<&str> = HashSet::new();

        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item
                && let oxc_ast::ast::JSXAttributeName::Identifier(ident) = &attr.name
            {
                let name = ident.name.as_str();
                if !seen.insert(name) {
                    let start = attr.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: format!("Duplicate prop `{name}`"),
                    });
                }
            }
        }
    }
}
