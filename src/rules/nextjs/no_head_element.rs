use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoHeadElement;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-head-element",
    default_severity: Severity::Warning,
    category: "nextjs",
    description: "Use `next/head` (`<Head>`) instead of `<head>`",
};

impl Rule for NoHeadElement {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = HeadCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct HeadCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for HeadCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let is_head = matches!(
            &el.name,
            oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "head"
        );
        if is_head {
            let start = el.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Use `next/head` (`<Head />`) instead of `<head>`".to_string(),
            });
        }
    }
}
