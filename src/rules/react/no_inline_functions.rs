use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoInlineFunctions;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-inline-functions",
    default_severity: Severity::Warning,
    category: "react",
    description: "Avoid inline function definitions in JSX props",
};

impl Rule for NoInlineFunctions {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = InlineFnCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct InlineFnCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for InlineFnCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item
                && let Some(val) = &attr.value
                    && let oxc_ast::ast::JSXAttributeValue::ExpressionContainer(container) = val {
                        let is_inline = matches!(
                            &container.expression,
                            oxc_ast::ast::JSXExpression::ArrowFunctionExpression(_)
                                | oxc_ast::ast::JSXExpression::FunctionExpression(_)
                        );
                        if is_inline {
                            let start = attr.span.start as usize;
                            let line = self.source[..start].lines().count().max(1);
                            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                            let name = if let oxc_ast::ast::JSXAttributeName::Identifier(id) = &attr.name {
                                id.name.as_str()
                            } else {
                                "prop"
                            };
                            self.findings.push(RuleFinding {
                                line,
                                column: col + 1,
                                message: format!("Avoid inline function for `{name}` prop — extract to a named handler"),
                            });
                        }
                    }
        }
    }
}
