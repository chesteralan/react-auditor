use oxc_ast::ast::{Program, StringLiteral};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoScriptUrl;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-script-url",
    default_severity: Severity::Error,
    category: "security",
    description: "No `javascript:` URLs in links",
};

impl Rule for NoScriptUrl {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ScriptUrlCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct ScriptUrlCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ScriptUrlCollector<'a> {
    fn visit_string_literal(&mut self, s: &StringLiteral) {
        let val = s.value.as_str();
        if val.to_lowercase().starts_with("javascript:") {
            let start = s.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Unexpected `javascript:` URL — security risk".to_string(),
            });
        }
    }

    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item
                && let Some(val) = &attr.value
                    && let oxc_ast::ast::JSXAttributeValue::StringLiteral(s) = val
                        && s.value.as_str().to_lowercase().starts_with("javascript:") {
                            let start = attr.span.start as usize;
                            let line = self.source[..start].lines().count().max(1);
                            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                            self.findings.push(RuleFinding {
                                line,
                                column: col + 1,
                                message: "Unexpected `javascript:` URL in JSX attribute — security risk".to_string(),
                            });
                        }
        }
    }
}
