use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoExplicitAny;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-explicit-any",
    default_severity: Severity::Error,
    category: "typescript",
    description: "Avoid explicit `any` type annotations or assertions",
};

impl Rule for NoExplicitAny {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ExplicitAnyCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ExplicitAnyCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> ExplicitAnyCollector<'a> {
    fn add_finding(&mut self, start: usize) {
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: "Unexpected explicit `any` type — use `unknown` or a proper type".to_string(),
        });
    }
}

impl<'a> Visit<'a> for ExplicitAnyCollector<'a> {
    fn visit_ts_type_annotation(&mut self, ann: &oxc_ast::ast::TSTypeAnnotation<'a>) {
        if is_any_type(&ann.type_annotation) {
            self.add_finding(ann.span.start as usize);
        }
    }

    fn visit_ts_as_expression(&mut self, expr: &oxc_ast::ast::TSAsExpression<'a>) {
        if is_any_type(&expr.type_annotation) {
            self.add_finding(expr.span.start as usize);
        }
    }

    fn visit_ts_type_assertion(&mut self, expr: &oxc_ast::ast::TSTypeAssertion<'a>) {
        if is_any_type(&expr.type_annotation) {
            self.add_finding(expr.span.start as usize);
        }
    }
}

fn is_any_type(ts_type: &oxc_ast::ast::TSType) -> bool {
    matches!(ts_type, oxc_ast::ast::TSType::TSAnyKeyword(_))
}
