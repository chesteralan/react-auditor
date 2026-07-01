use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct AriaValid;

const RULE_META: RuleMeta = RuleMeta {
    id: "aria-valid",
    default_severity: Severity::Error,
    category: "accessibility",
    description: "ARIA attributes must be valid",
};

const VALID_ARIA: &[&str] = &[
    "aria-activedescendant", "aria-atomic", "aria-autocomplete",
    "aria-busy", "aria-checked", "aria-colcount", "aria-colindex",
    "aria-colspan", "aria-controls", "aria-current", "aria-describedby",
    "aria-details", "aria-disabled", "aria-dropeffect", "aria-errormessage",
    "aria-expanded", "aria-flowto", "aria-grabbed", "aria-haspopup",
    "aria-hidden", "aria-invalid", "aria-keyshortcuts", "aria-label",
    "aria-labelledby", "aria-level", "aria-live", "aria-modal",
    "aria-multiline", "aria-multiselectable", "aria-orientation",
    "aria-owns", "aria-placeholder", "aria-posinset", "aria-pressed",
    "aria-readonly", "aria-relevant", "aria-required", "aria-roledescription",
    "aria-rowcount", "aria-rowindex", "aria-rowspan", "aria-selected",
    "aria-setsize", "aria-sort", "aria-valuemax", "aria-valuemin",
    "aria-valuenow", "aria-valuetext",
];

impl Rule for AriaValid {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = AriaCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct AriaCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> AriaCollector<'a> {
    fn add_finding(&mut self, start: usize, msg: String) {
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding { line, column: col + 1, message: msg });
    }
}

impl<'a> Visit<'a> for AriaCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item
                && let oxc_ast::ast::JSXAttributeName::Identifier(id) = &attr.name {
                    let name = id.name.as_str();
                    if name.starts_with("aria-") && !VALID_ARIA.contains(&name) {
                        self.add_finding(attr.span.start as usize,
                            format!("Unknown ARIA attribute `{name}`"));
                    }
                }
        }
    }
}
