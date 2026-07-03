use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoArrayIndexKey;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-array-index-key",
    default_severity: Severity::Warning,
    category: "react",
    description: "Avoid using array index as key",
};

fn is_index_name(name: &str) -> bool {
    matches!(name, "index" | "i" | "j" | "idx" | "ind" | "key")
}

impl Rule for NoArrayIndexKey {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ArrayIndexKeyCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ArrayIndexKeyCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ArrayIndexKeyCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item {
                let is_key = matches!(&attr.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "key");
                if !is_key {
                    continue;
                }
                if let Some(val) = &attr.value
                    && let oxc_ast::ast::JSXAttributeValue::ExpressionContainer(container) = val
                    && matches!(&container.expression, oxc_ast::ast::JSXExpression::Identifier(ident) if is_index_name(ident.name.as_str()))
                {
                    let start = attr.span.start as usize;
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    let line = self.source[..start].lines().count().max(1);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Avoid using array index as `key` — prefer a stable unique ID"
                            .to_string(),
                    });
                }
            }
        }
    }
}
