use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_syntax::scope::ScopeFlags;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct ConsistentComponentNaming;

const RULE_META: RuleMeta = RuleMeta {
    id: "consistent-component-naming",
    default_severity: Severity::Warning,
    category: "react",
    description: "Component names should be PascalCase, hooks should be camelCase",
};

impl Rule for ConsistentComponentNaming {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = NamingCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct NamingCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for NamingCollector<'a> {
    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, _flags: ScopeFlags) {
        if let Some(id) = &func.id {
            let name = id.name.as_str();

            if name.starts_with("use") && name.len() > 3 {
                // Hooks should be camelCase (useFoo), not PascalCase
                let rest = &name[3..];
                if !rest.is_empty() && rest.chars().next().is_some_and(|c| c.is_uppercase()) {
                    // This is a valid hook name like `useState` - skip
                    return;
                }
                return;
            }

            // Check if function returns JSX (heuristic: check body for JSX)
            let returns_jsx = func.body.as_ref().is_some_and(|body| {
                body.statements.iter().any(|stmt| contains_jsx(stmt))
            });

            if returns_jsx {
                let first_char = name.chars().next().unwrap_or(' ');
                if first_char.is_lowercase() {
                    let start = id.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: format!("Component `{name}` should be PascalCase"),
                    });
                }
            }
        }
    }
}

fn contains_jsx(stmt: &oxc_ast::ast::Statement) -> bool {
    match stmt {
        oxc_ast::ast::Statement::ReturnStatement(ret) => {
            ret.argument.as_ref().is_some_and(|arg| is_jsx_expression(arg))
        }
        oxc_ast::ast::Statement::ExpressionStatement(expr) => is_jsx_expression(&expr.expression),
        oxc_ast::ast::Statement::BlockStatement(block) => {
            block.body.iter().any(|s| contains_jsx(s))
        }
        _ => false,
    }
}

fn is_jsx_expression(expr: &oxc_ast::ast::Expression) -> bool {
    matches!(
        expr,
        oxc_ast::ast::Expression::JSXElement(_) | oxc_ast::ast::Expression::JSXFragment(_)
    )
}
