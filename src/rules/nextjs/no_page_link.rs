use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoPageLink;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-page-link",
    default_severity: Severity::Warning,
    category: "nextjs",
    description: "Use `next/link` instead of `<a>` for internal navigation",
};

impl Rule for NoPageLink {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = PageLinkCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct PageLinkCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for PageLinkCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let is_a = matches!(
            &el.name,
            oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "a"
        );
        if !is_a {
            return;
        }

        let mut href_value: Option<String> = None;
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item
                && let oxc_ast::ast::JSXAttributeName::Identifier(ident) = &attr.name
                && ident.name.as_str() == "href"
            {
                if let Some(val) = &attr.value
                    && let oxc_ast::ast::JSXAttributeValue::StringLiteral(s) = val
                {
                    href_value = Some(s.value.to_string());
                    break;
                }
                if let Some(val) = &attr.value
                    && let oxc_ast::ast::JSXAttributeValue::ExpressionContainer(container) = val
                    && let oxc_ast::ast::JSXExpression::StringLiteral(s) = &container.expression
                {
                    href_value = Some(s.value.to_string());
                    break;
                }
            }
        }

        if let Some(href) = href_value
            && (href.starts_with('/') || href.starts_with('.'))
        {
            let start = el.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: format!(
                    "Use `next/link` (`<Link href=\"{href}\">`) instead of `<a>` for internal navigation"
                ),
            });
        }
    }
}
