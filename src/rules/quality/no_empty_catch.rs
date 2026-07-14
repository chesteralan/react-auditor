use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoEmptyCatch;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-empty-catch",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Don't ignore caught errors — rethrow or handle them",
};

impl Rule for NoEmptyCatch {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = EmptyCatchCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct EmptyCatchCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for EmptyCatchCollector<'a> {
    fn visit_catch_clause(&mut self, clause: &oxc_ast::ast::CatchClause<'a>) {
        if clause.body.body.is_empty() {
            let start = clause.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Empty catch block — don't ignore errors, rethrow or handle them"
                    .to_string(),
            });
        }
    }
}
