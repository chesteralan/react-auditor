use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk;
use oxc_semantic::Semantic;
use oxc_syntax::scope::ScopeFlags;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct Complexity;

const RULE_META: RuleMeta = RuleMeta {
    id: "complexity",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Cyclomatic complexity should not exceed 10",
};

const MAX_COMPLEXITY: usize = 10;

impl Rule for Complexity {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ComplexityCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ComplexityCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ComplexityCollector<'a> {
    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, _flags: ScopeFlags) {
        if let Some(body) = &func.body {
            let score = count_complexity(&body.statements);
            if score > MAX_COMPLEXITY {
                let name = func
                    .id
                    .as_ref()
                    .map(|id| id.name.as_str())
                    .unwrap_or("anonymous");
                let start = func.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: format!(
                        "Function `{name}` has complexity {score}, max {MAX_COMPLEXITY}"
                    ),
                });
            }
        }
        walk::walk_function(self, func, _flags);
    }

    fn visit_arrow_function_expression(
        &mut self,
        func: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        let score = count_complexity(&func.body.statements);
        if score > MAX_COMPLEXITY {
            let start = func.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: format!("Arrow function has complexity {score}, max {MAX_COMPLEXITY}"),
            });
        }
        walk::walk_arrow_function_expression(self, func);
    }
}

fn count_statement(stmt: &oxc_ast::ast::Statement) -> usize {
    match stmt {
        oxc_ast::ast::Statement::IfStatement(i) => {
            let mut score = 1;
            if i.alternate.is_some() {
                score += 1;
            }
            score += count_statement(&i.consequent);
            if let Some(alt) = &i.alternate {
                score += count_statement(alt);
            }
            score
        }
        oxc_ast::ast::Statement::ForStatement(f) => 1 + count_statement(&f.body),
        oxc_ast::ast::Statement::ForInStatement(f) => 1 + count_statement(&f.body),
        oxc_ast::ast::Statement::ForOfStatement(f) => 1 + count_statement(&f.body),
        oxc_ast::ast::Statement::WhileStatement(w) => 1 + count_statement(&w.body),
        oxc_ast::ast::Statement::DoWhileStatement(d) => 1 + count_statement(&d.body),
        oxc_ast::ast::Statement::SwitchStatement(s) => s
            .cases
            .iter()
            .map(|case| 1 + count_complexity(&case.consequent))
            .sum(),
        oxc_ast::ast::Statement::TryStatement(t) => {
            let mut score = 0;
            if let Some(handler) = &t.handler {
                score += 1;
                score += count_complexity(&handler.body.body);
            }
            if let Some(ref finalizer) = t.finalizer {
                score += count_complexity(&finalizer.body);
            }
            score
        }
        oxc_ast::ast::Statement::BlockStatement(b) => count_complexity(&b.body),
        _ => 0,
    }
}

fn count_complexity(stmts: &[oxc_ast::ast::Statement]) -> usize {
    let mut score = 1;
    for stmt in stmts {
        score += count_statement(stmt);
    }
    score
}
