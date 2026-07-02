use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct ClickEventsHaveKeyEvents;

const RULE_META: RuleMeta = RuleMeta {
    id: "click-events-have-key-events",
    default_severity: Severity::Warning,
    category: "accessibility",
    description: "Elements with onClick must have a keyboard event handler",
};

impl Rule for ClickEventsHaveKeyEvents {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ClickKeyCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ClickKeyCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

const KEY_EVENTS: &[&str] = &["onKeyDown", "onKeyUp", "onKeyPress"];

impl<'a> Visit<'a> for ClickKeyCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let mut has_click = false;
        let mut has_keyboard = false;

        for attr in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr {
                if let oxc_ast::ast::JSXAttributeName::Identifier(id) = &a.name {
                    let name = id.name.as_str();
                    if name == "onClick" {
                        has_click = true;
                    } else if KEY_EVENTS.contains(&name) {
                        has_keyboard = true;
                    }
                }
            }
        }

        if has_click && !has_keyboard {
            let start = el.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Elements with onClick must also have onKeyDown, onKeyUp, or onKeyPress"
                    .to_string(),
            });
        }
    }
}
