use std::collections::HashMap;

use oxc_ast::ast::Program;
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoAmbiguousLabels;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-ambiguous-labels",
    default_severity: Severity::Warning,
    category: "accessibility",
    description: "No duplicate or ambiguous label text",
};

impl Rule for NoAmbiguousLabels {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = AmbiguousLabelCollector {
            findings: Vec::new(),
            source: source_text,
            label_texts: HashMap::new(),
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct AmbiguousLabelCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    label_texts: HashMap<String, Vec<usize>>,
}

impl<'a> AmbiguousLabelCollector<'a> {
    fn add_finding(&mut self, start: usize, label: &str) {
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: format!(
                "Duplicate label text \"{label}\" — labels must be unique for accessibility"
            ),
        });
    }
}

impl<'a> Visit<'a> for AmbiguousLabelCollector<'a> {
    fn visit_jsx_element(&mut self, el: &oxc_ast::ast::JSXElement<'a>) {
        let is_label = matches!(
            &el.opening_element.name,
            oxc_ast::ast::JSXElementName::Identifier(ident)
                if ident.name.as_str() == "label"
        );

        if is_label {
            let mut text = String::new();
            for child in &el.children {
                if let oxc_ast::ast::JSXChild::Text(t) = child {
                    let trimmed = t.value.trim();
                    if !trimmed.is_empty() {
                        text.push_str(trimmed);
                        text.push(' ');
                    }
                }
            }
            let label = text.trim().to_lowercase();
            if !label.is_empty() {
                if let Some(positions) = self.label_texts.get(&label) {
                    if positions.len() == 1 {
                        self.add_finding(positions[0], &label);
                    }
                    self.add_finding(el.opening_element.span.start as usize, &label);
                }
                self.label_texts
                    .entry(label)
                    .or_default()
                    .push(el.opening_element.span.start as usize);
            }
        }

        walk::walk_jsx_element(self, el);
    }
}
