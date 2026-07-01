use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoInlineStyles;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-inline-styles",
    default_severity: Severity::Warning,
    category: "react",
    description: "Avoid inline `style` prop — use CSS classes",
};

impl Rule for NoInlineStyles {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = InlineStyleCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct InlineStyleCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for InlineStyleCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item {
                let is_style = matches!(
                    &attr.name,
                    oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "style"
                );

                if is_style && attr.value.is_some() {
                    let start = attr.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Avoid inline `style` prop, use a CSS class instead".to_string(),
                    });
                }
            }
        }
    }
}
