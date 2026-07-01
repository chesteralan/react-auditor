use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoUnsanitizedInput;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-unsanitized-input",
    default_severity: Severity::Error,
    category: "security",
    description: "Sanitize user input before DOM insertion",
};

const DANGEROUS_ASSIGNMENTS: &[&str] = &[
    "innerHTML", "outerHTML",
];

impl Rule for NoUnsanitizedInput {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = UnsanitizedCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct UnsanitizedCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> UnsanitizedCollector<'a> {
    fn add_finding(&mut self, start: usize, msg: String) {
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding { line, column: col + 1, message: msg });
    }
}

impl<'a> Visit<'a> for UnsanitizedCollector<'a> {
    fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
        let prop = match &expr.left {
            oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) => Some(m.property.name.as_str()),
            oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(_) => {
                None
            }
            _ => None,
        };

        if let Some(prop) = prop
            && DANGEROUS_ASSIGNMENTS.contains(&prop) {
                self.add_finding(expr.span.start as usize,
                    format!("Direct assignment to `.{prop}` — sanitize user input first"));
            }
    }
}
