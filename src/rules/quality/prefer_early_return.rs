use oxc_ast::ast::Program;
use oxc_ast_visit::walk::walk_statement;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct PreferEarlyReturn;

const RULE_META: RuleMeta = RuleMeta {
    id: "prefer-early-return",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Prefer early returns over nested if-else",
};

impl Rule for PreferEarlyReturn {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = EarlyReturnCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct EarlyReturnCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for EarlyReturnCollector<'a> {
    fn visit_statement(&mut self, stmt: &oxc_ast::ast::Statement<'a>) {
        if let oxc_ast::ast::Statement::IfStatement(if_stmt) = stmt
            && let Some(alt) = &if_stmt.alternate {
                let consequent_is_return = is_single_return(&if_stmt.consequent);

                if consequent_is_return {
                    let start = if_stmt.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Prefer early return over if-else when if block only returns".to_string(),
                    });
                    let _ = alt;
                }
            }
        walk_statement(self, stmt);
    }
}

fn is_single_return(stmt: &oxc_ast::ast::Statement) -> bool {
    match stmt {
        oxc_ast::ast::Statement::ReturnStatement(_) => true,
        oxc_ast::ast::Statement::BlockStatement(block) => {
            block.body.len() == 1 && matches!(block.body.first(), Some(oxc_ast::ast::Statement::ReturnStatement(_)))
        }
        _ => false,
    }
}
