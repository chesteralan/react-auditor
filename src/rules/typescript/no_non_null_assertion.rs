use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoNonNullAssertion;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-non-null-assertion",
    default_severity: Severity::Warning,
    category: "typescript",
    description: "Avoid `!` non-null assertion operator",
};

impl Rule for NoNonNullAssertion {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = NonNullCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct NonNullCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for NonNullCollector<'a> {
    fn visit_ts_non_null_expression(&mut self, expr: &oxc_ast::ast::TSNonNullExpression<'a>) {
        let start = expr.span.start as usize;
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: "Unexpected non-null assertion `!` — use optional chaining or type guard"
                .to_string(),
        });
    }
}
