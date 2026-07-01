use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoImgElement;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-img-element",
    default_severity: Severity::Warning,
    category: "nextjs",
    description: "Use `next/image` instead of `<img>` for optimized images",
};

impl Rule for NoImgElement {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ImgCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ImgCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ImgCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let is_img = matches!(
            &el.name,
            oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "img"
        );
        if is_img {
            let start = el.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Use `next/image` (`<Image />`) instead of `<img>` for optimized images"
                    .to_string(),
            });
        }
    }
}
