use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk;
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
        let mut collector = ReturnCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ReturnCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

fn has_return_with_value(stmts: &[oxc_ast::ast::Statement]) -> bool {
    stmts.iter().any(|s| match s {
        oxc_ast::ast::Statement::ReturnStatement(r) => r.argument.is_some(),
        oxc_ast::ast::Statement::BlockStatement(b) => has_return_with_value(&b.body),
        oxc_ast::ast::Statement::IfStatement(i) => {
            let cons = has_return_with_value(std::slice::from_ref(&i.consequent));
            let alt = i
                .alternate
                .as_ref()
                .is_some_and(|a| has_return_with_value(std::slice::from_ref(a)));
            cons || alt
        }
        oxc_ast::ast::Statement::SwitchStatement(s) => s
            .cases
            .iter()
            .any(|case| has_return_with_value(&case.consequent)),
        oxc_ast::ast::Statement::ForStatement(f) => {
            has_return_with_value(std::slice::from_ref(&f.body))
        }
        oxc_ast::ast::Statement::ForInStatement(f) => {
            has_return_with_value(std::slice::from_ref(&f.body))
        }
        oxc_ast::ast::Statement::ForOfStatement(f) => {
            has_return_with_value(std::slice::from_ref(&f.body))
        }
        oxc_ast::ast::Statement::WhileStatement(w) => {
            has_return_with_value(std::slice::from_ref(&w.body))
        }
        oxc_ast::ast::Statement::DoWhileStatement(d) => {
            has_return_with_value(std::slice::from_ref(&d.body))
        }
        oxc_ast::ast::Statement::TryStatement(t) => {
            let try_has = has_return_with_value(&t.block.body);
            let catch_has = t
                .handler
                .as_ref()
                .is_some_and(|h| has_return_with_value(&h.body.body));
            let finally_has = t
                .finalizer
                .as_ref()
                .is_some_and(|f| has_return_with_value(&f.body));
            try_has || catch_has || finally_has
        }
        _ => false,
    })
}

fn has_return_without_value(stmts: &[oxc_ast::ast::Statement]) -> bool {
    stmts.iter().any(|s| match s {
        oxc_ast::ast::Statement::ReturnStatement(r) => r.argument.is_none(),
        oxc_ast::ast::Statement::BlockStatement(b) => has_return_without_value(&b.body),
        oxc_ast::ast::Statement::IfStatement(i) => {
            let cons = has_return_without_value(std::slice::from_ref(&i.consequent));
            let alt = i
                .alternate
                .as_ref()
                .is_some_and(|a| has_return_without_value(std::slice::from_ref(a)));
            cons || alt
        }
        oxc_ast::ast::Statement::SwitchStatement(s) => s
            .cases
            .iter()
            .any(|case| has_return_without_value(&case.consequent)),
        oxc_ast::ast::Statement::ForStatement(f) => {
            has_return_without_value(std::slice::from_ref(&f.body))
        }
        oxc_ast::ast::Statement::ForInStatement(f) => {
            has_return_without_value(std::slice::from_ref(&f.body))
        }
        oxc_ast::ast::Statement::ForOfStatement(f) => {
            has_return_without_value(std::slice::from_ref(&f.body))
        }
        oxc_ast::ast::Statement::WhileStatement(w) => {
            has_return_without_value(std::slice::from_ref(&w.body))
        }
        oxc_ast::ast::Statement::DoWhileStatement(d) => {
            has_return_without_value(std::slice::from_ref(&d.body))
        }
        oxc_ast::ast::Statement::TryStatement(t) => {
            let try_has = has_return_without_value(&t.block.body);
            let catch_has = t
                .handler
                .as_ref()
                .is_some_and(|h| has_return_without_value(&h.body.body));
            let finally_has = t
                .finalizer
                .as_ref()
                .is_some_and(|f| has_return_without_value(&f.body));
            try_has || catch_has || finally_has
        }
        _ => false,
    })
}

fn has_any_return(stmts: &[oxc_ast::ast::Statement]) -> bool {
    stmts.iter().any(|s| match s {
        oxc_ast::ast::Statement::ReturnStatement(_) => true,
        oxc_ast::ast::Statement::BlockStatement(b) => has_any_return(&b.body),
        oxc_ast::ast::Statement::IfStatement(i) => {
            has_any_return(std::slice::from_ref(&i.consequent))
                || i.alternate
                    .as_ref()
                    .is_some_and(|a| has_any_return(std::slice::from_ref(a)))
        }
        oxc_ast::ast::Statement::SwitchStatement(s) => {
            s.cases.iter().any(|case| has_any_return(&case.consequent))
        }
        oxc_ast::ast::Statement::ForStatement(f) => has_any_return(std::slice::from_ref(&f.body)),
        oxc_ast::ast::Statement::ForInStatement(f) => has_any_return(std::slice::from_ref(&f.body)),
        oxc_ast::ast::Statement::ForOfStatement(f) => has_any_return(std::slice::from_ref(&f.body)),
        oxc_ast::ast::Statement::WhileStatement(w) => has_any_return(std::slice::from_ref(&w.body)),
        oxc_ast::ast::Statement::DoWhileStatement(d) => {
            has_any_return(std::slice::from_ref(&d.body))
        }
        oxc_ast::ast::Statement::TryStatement(t) => {
            has_any_return(&t.block.body)
                || t.handler
                    .as_ref()
                    .is_some_and(|h| has_any_return(&h.body.body))
                || t.finalizer
                    .as_ref()
                    .is_some_and(|f| has_any_return(&f.body))
        }
        _ => false,
    })
}

impl<'a> Visit<'a> for ReturnCollector<'a> {
    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, _flags: ScopeFlags) {
        if let Some(body) = &func.body
            && has_any_return(&body.statements)
        {
            let has_value = has_return_with_value(&body.statements);
            let has_no_value = has_return_without_value(&body.statements);

            if has_value && has_no_value {
                let start = func.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: "Function inconsistently returns a value and returns without a value"
                        .to_string(),
                });
            }
        }
        walk::walk_function(self, func, _flags);
    }

    fn visit_arrow_function_expression(
        &mut self,
        func: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        if func.expression {
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
                    message:
                        "Arrow function inconsistently returns a value and returns without a value"
                            .to_string(),
                });
            }
        }
    }
}
