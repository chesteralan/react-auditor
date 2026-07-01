use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoUnsafeIframe;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-unsafe-iframe",
    default_severity: Severity::Warning,
    category: "security",
    description: "iframes should include `sandbox` and `title` attributes",
};

impl Rule for NoUnsafeIframe {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = UnsafeIframeCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct UnsafeIframeCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for UnsafeIframeCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let is_iframe = matches!(
            &el.name,
            oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "iframe"
        );
        if !is_iframe {
            return;
        }

        let mut has_sandbox = false;
        let mut has_title = false;

        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item
                && let oxc_ast::ast::JSXAttributeName::Identifier(ident) = &attr.name
            {
                match ident.name.as_str() {
                    "sandbox" => has_sandbox = true,
                    "title" => has_title = true,
                    _ => {}
                }
            }
        }

        let start = el.span.start as usize;
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

        if !has_sandbox {
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "`<iframe>` is missing a `sandbox` attribute for security isolation"
                    .to_string(),
            });
        }
        if !has_title {
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "`<iframe>` is missing a `title` attribute for accessibility".to_string(),
            });
        }
    }
}
