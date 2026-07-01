use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct PreferFragments;

const RULE_META: RuleMeta = RuleMeta {
    id: "prefer-fragments",
    default_severity: Severity::Warning,
    category: "performance",
    description: "Use `<></>` over unnecessary wrapper divs",
};

impl Rule for PreferFragments {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = FragmentCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct FragmentCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for FragmentCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let is_div = matches!(&el.name, oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "div");
        if !is_div {
            return;
        }
        let has_class = el.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr {
                matches!(&a.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "className" || id.name.as_str() == "class")
            } else {
                false
            }
        });
        if has_class {
            return;
        }
        let has_style = el.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr {
                matches!(&a.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "style")
            } else {
                false
            }
        });
        if !has_class && !has_style {
            let start = el.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Unnecessary `<div>` wrapper — use `<></>` Fragment instead".to_string(),
            });
        }
    }
}
