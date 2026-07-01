use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoBindInJsx;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-bind-in-jsx",
    default_severity: Severity::Warning,
    category: "performance",
    description: "Avoid `.bind()` or arrow functions in JSX props",
};

impl Rule for NoBindInJsx {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = BindCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct BindCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for BindCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item
                && let Some(val) = &attr.value
                    && let oxc_ast::ast::JSXAttributeValue::ExpressionContainer(container) = val {
                        let has_bind = match &container.expression {
                            oxc_ast::ast::JSXExpression::CallExpression(call) => {
                                if let Some(member) = call.callee.as_member_expression() {
                                    member.static_property_name() == Some("bind")
                                } else {
                                    false
                                }
                            }
                            _ => false,
                        };
                        if has_bind {
                            let start = attr.span.start as usize;
                            let line = self.source[..start].lines().count().max(1);
                            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                            self.findings.push(RuleFinding {
                                line,
                                column: col + 1,
                                message: "Avoid `.bind()` in JSX – use a class property or useCallback".to_string(),
                            });
                        }
                    }
        }
    }
}
