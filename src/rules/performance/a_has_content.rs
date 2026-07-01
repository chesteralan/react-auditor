use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct AHasContent;

const RULE_META: RuleMeta = RuleMeta {
    id: "a-has-content",
    default_severity: Severity::Warning,
    category: "accessibility",
    description: "Anchor and button elements should have text content or an aria-label",
};

impl Rule for AHasContent {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = AHasContentCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct AHasContentCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for AHasContentCollector<'a> {
    fn visit_jsx_element(&mut self, el: &oxc_ast::ast::JSXElement<'a>) {
        let is_target = matches!(
            &el.opening_element.name,
            oxc_ast::ast::JSXElementName::Identifier(id)
                if matches!(id.name.as_str(), "a" | "button")
        );

        if is_target {
            let mut has_aria = false;
            for attr_item in &el.opening_element.attributes {
                if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item
                    && let oxc_ast::ast::JSXAttributeName::Identifier(ident) = &attr.name
                    && matches!(ident.name.as_str(), "aria-label" | "aria-labelledby")
                {
                    has_aria = true;
                    break;
                }
            }

            if !has_aria {
                let has_text_content = el.children.iter().any(|child| match child {
                    oxc_ast::ast::JSXChild::Text(text) => {
                        let trimmed = text.value.trim();
                        !trimmed.is_empty()
                    }
                    oxc_ast::ast::JSXChild::Element(_)
                    | oxc_ast::ast::JSXChild::ExpressionContainer(_) => true,
                    _ => false,
                });

                if !has_text_content {
                    let start = el.opening_element.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    let tag = if matches!(&el.opening_element.name,
                        oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "a"
                    ) {
                        "link"
                    } else {
                        "button"
                    };
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: format!(
                            "`<{tag}>` element has no text content or `aria-label` attribute"
                        ),
                    });
                }
            }
        }

        walk::walk_jsx_element(self, el);
    }
}
