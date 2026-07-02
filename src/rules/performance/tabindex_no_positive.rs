use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct TabindexNoPositive;

const RULE_META: RuleMeta = RuleMeta {
    id: "tabindex-no-positive",
    default_severity: Severity::Error,
    category: "accessibility",
    description: "Avoid positive tabIndex values; only 0 and -1 are valid",
};

impl Rule for TabindexNoPositive {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = TabindexCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct TabindexCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for TabindexCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr
                && let oxc_ast::ast::JSXAttributeName::Identifier(id) = &a.name
                && id.name.as_str() == "tabIndex"
                && let Some(value) = &a.value
            {
                if let oxc_ast::ast::JSXAttributeValue::ExpressionContainer(expr) = value
                    && let Some(oxc_ast::ast::Expression::NumericLiteral(n)) =
                        expr.expression.as_expression()
                    && n.value > 0.0
                {
                    let start = el.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start
                        - self.source[..start]
                            .rfind('\n')
                            .map(|i| i + 1)
                            .unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: format!(
                            "tabIndex={} is positive; use 0 or -1 only",
                            n.value as i64
                        ),
                    });
                }
                if let oxc_ast::ast::JSXAttributeValue::StringLiteral(s) = value
                    && let Ok(n) = s.value.parse::<i64>()
                    && n > 0
                {
                    let start = el.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start
                        - self.source[..start]
                            .rfind('\n')
                            .map(|i| i + 1)
                            .unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: format!(
                            "tabIndex=\"{}\" is positive; use 0 or -1 only",
                            n
                        ),
                    });
                }
            }
        }
    }
}
