use oxc_ast::ast::Program;
use oxc_ast_visit::walk;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_syntax::scope::ScopeFlags;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct ConsistentReturn;

const RULE_META: RuleMeta = RuleMeta {
    id: "consistent-return",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Functions should consistently return a value or not",
};

impl Rule for ConsistentReturn {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ReturnCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct ReturnCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

fn has_return_with_value(stmts: &[oxc_ast::ast::Statement]) -> bool {
    stmts.iter().any(|s| {
        if let oxc_ast::ast::Statement::ReturnStatement(r) = s {
            r.argument.is_some()
        } else if let oxc_ast::ast::Statement::BlockStatement(b) = s {
            has_return_with_value(&b.body)
        } else if let oxc_ast::ast::Statement::IfStatement(i) = s {
            let cons = has_return_with_value(
                std::slice::from_ref(&i.consequent),
            );
            let alt = i.alternate.as_ref().is_some_and(|a| {
                has_return_with_value(std::slice::from_ref(a))
            });
            cons || alt
        } else {
            false
        }
    })
}

fn has_return_without_value(stmts: &[oxc_ast::ast::Statement]) -> bool {
    stmts.iter().any(|s| {
        if let oxc_ast::ast::Statement::ReturnStatement(r) = s {
            r.argument.is_none()
        } else if let oxc_ast::ast::Statement::BlockStatement(b) = s {
            has_return_without_value(&b.body)
        } else {
            false
        }
    })
}

fn has_any_return(stmts: &[oxc_ast::ast::Statement]) -> bool {
    stmts.iter().any(|s| {
        if matches!(s, oxc_ast::ast::Statement::ReturnStatement(_)) {
            true
        } else if let oxc_ast::ast::Statement::BlockStatement(b) = s {
            has_any_return(&b.body)
        } else {
            false
        }
    })
}

impl<'a> Visit<'a> for ReturnCollector<'a> {
    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, _flags: ScopeFlags) {
        if let Some(body) = &func.body
            && has_any_return(&body.statements) {
                let has_value = has_return_with_value(&body.statements);
                let has_no_value = has_return_without_value(&body.statements);

                if has_value && has_no_value {
                    let start = func.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Function inconsistently returns a value and returns without a value".to_string(),
                    });
                }
            }
        walk::walk_function(self, func, _flags);
    }

    fn visit_arrow_function_expression(&mut self, func: &oxc_ast::ast::ArrowFunctionExpression<'a>) {
        if func.expression {
            // Expression bodies always return a value
            return;
        }
        if has_any_return(&func.body.statements) {
            let has_value = has_return_with_value(&func.body.statements);
            let has_no_value = has_return_without_value(&func.body.statements);

            if has_value && has_no_value {
                let start = func.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: "Arrow function inconsistently returns a value and returns without a value".to_string(),
                });
            }
        }
    }
}
