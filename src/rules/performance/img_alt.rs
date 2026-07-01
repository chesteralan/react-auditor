use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct ImgAlt;

const RULE_META: RuleMeta = RuleMeta {
    id: "img-alt",
    default_severity: Severity::Error,
    category: "accessibility",
    description: "Images must have `alt` text",
};

impl Rule for ImgAlt {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ImgAltCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct ImgAltCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ImgAltCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let is_img = matches!(&el.name, oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "img");
        if !is_img {
            return;
        }
        let has_alt = el.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr {
                matches!(&a.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "alt")
            } else {
                false
            }
        });
        if !has_alt {
            let start = el.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Missing `alt` attribute on `<img>`".to_string(),
            });
        }
    }
}
