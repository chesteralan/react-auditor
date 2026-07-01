use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoInsecureProtocol;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-insecure-protocol",
    default_severity: Severity::Warning,
    category: "security",
    description: "Avoid `http://` URLs, use `https://`",
};

impl Rule for NoInsecureProtocol {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = InsecureProtocolCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct InsecureProtocolCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> InsecureProtocolCollector<'a> {
    fn add_finding(&mut self, start: usize, msg: String) {
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: msg,
        });
    }
}

impl<'a> Visit<'a> for InsecureProtocolCollector<'a> {
    fn visit_string_literal(&mut self, s: &oxc_ast::ast::StringLiteral) {
        let val = s.value.as_str();
        if val.starts_with("http://") {
            self.add_finding(
                s.span.start as usize,
                "Use `https://` instead of `http://` in URL".to_string(),
            );
        }
    }

    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item
                && let Some(val) = &attr.value
                && let oxc_ast::ast::JSXAttributeValue::StringLiteral(s) = val
                && s.value.as_str().starts_with("http://")
            {
                self.add_finding(
                    attr.span.start as usize,
                    "Use `https://` instead of `http://` in JSX attribute".to_string(),
                );
            }
        }
    }
}
