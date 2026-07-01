use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoTypeAssertion;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-type-assertion",
    default_severity: Severity::Warning,
    category: "typescript",
    description: "Prefer type inference over explicit `as` casts",
};

impl Rule for NoTypeAssertion {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = AssertionCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct AssertionCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for AssertionCollector<'a> {
    fn visit_ts_as_expression(&mut self, expr: &oxc_ast::ast::TSAsExpression<'a>) {
        let start = expr.span.start as usize;
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: "Unexpected type assertion `as` — prefer type inference or type guard"
                .to_string(),
        });
    }

    fn visit_ts_type_assertion(&mut self, expr: &oxc_ast::ast::TSTypeAssertion<'a>) {
        let start = expr.span.start as usize;
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: "Unexpected type assertion `<Type>value` — prefer type inference".to_string(),
        });
    }
}
