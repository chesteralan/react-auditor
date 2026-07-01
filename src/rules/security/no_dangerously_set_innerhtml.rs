use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoDangerouslySetInnerHtml;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-dangerously-set-innerhtml",
    default_severity: Severity::Error,
    category: "security",
    description: "Avoid `dangerouslySetInnerHTML`",
};

impl Rule for NoDangerouslySetInnerHtml {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = DangerousHtmlCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct DangerousHtmlCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for DangerousHtmlCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item {
                let is_dangerous = matches!(&attr.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "dangerouslySetInnerHTML");
                if is_dangerous {
                    let start = attr.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Avoid `dangerouslySetInnerHTML` — sanitize HTML with DOMPurify if needed".to_string(),
                    });
                }
            }
        }
    }
}
