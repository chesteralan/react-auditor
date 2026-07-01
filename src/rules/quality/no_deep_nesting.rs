use oxc_ast::ast::Program;
use oxc_ast_visit::walk::walk_statement;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::GetSpan;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoDeepNesting;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-deep-nesting",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Avoid nesting deeper than 4 levels",
};

const MAX_DEPTH: usize = 4;

impl Rule for NoDeepNesting {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = NestingCollector {
            findings: Vec::new(),
            source: source_text,
            depth: 0,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct NestingCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    depth: usize,
}

impl<'a> Visit<'a> for NestingCollector<'a> {
    fn visit_statement(&mut self, stmt: &oxc_ast::ast::Statement<'a>) {
        let is_nesting = matches!(
            stmt,
            oxc_ast::ast::Statement::IfStatement(_)
                | oxc_ast::ast::Statement::ForStatement(_)
                | oxc_ast::ast::Statement::ForInStatement(_)
                | oxc_ast::ast::Statement::ForOfStatement(_)
                | oxc_ast::ast::Statement::WhileStatement(_)
                | oxc_ast::ast::Statement::DoWhileStatement(_)
                | oxc_ast::ast::Statement::SwitchStatement(_)
        );

        if is_nesting {
            self.depth += 1;
            if self.depth > MAX_DEPTH {
                let start = stmt.span().start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: format!("Nesting depth {depth} exceeds max {MAX_DEPTH}", depth = self.depth),
                });
            }
            walk_statement(self, stmt);
            self.depth -= 1;
        } else {
            walk_statement(self, stmt);
        }
    }
}
