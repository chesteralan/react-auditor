use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoIndexKey;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-index-key",
    default_severity: Severity::Warning,
    category: "react",
    description: "Prefer stable IDs over array index as key",
};

impl Rule for NoIndexKey {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = IndexKeyCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct IndexKeyCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for IndexKeyCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item {
                let is_key = matches!(&attr.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "key");
                if !is_key {
                    continue;
                }
                if let Some(val) = &attr.value
                    && let oxc_ast::ast::JSXAttributeValue::ExpressionContainer(container) = val
                        && let oxc_ast::ast::JSXExpression::Identifier(ident) = &container.expression
                            && ident.name.as_str() == "index" {
                                let start = attr.span.start as usize;
                                let line = self.source[..start].lines().count().max(1);
                                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                                self.findings.push(RuleFinding {
                                    line,
                                    column: col + 1,
                                    message: "Avoid using array index as `key` — prefer a stable ID".to_string(),
                                });
                            }
            }
        }
    }
}
