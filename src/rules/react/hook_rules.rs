use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct HookRules;

const RULE_META: RuleMeta = RuleMeta {
    id: "hook-rules",
    default_severity: Severity::Error,
    category: "react",
    description: "Hooks must follow the Rules of Hooks",
};

const HOOK_PREFIXES: &[&str] = &["use"];

fn is_hook_call(name: &str) -> bool {
    HOOK_PREFIXES.iter().any(|p| name.starts_with(p))
        && name.len() > 3
        && name.as_bytes()[3].is_ascii_uppercase()
}

struct HookRuleCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    inside_loop: Vec<bool>,
    inside_condition: Vec<bool>,
    inside_nested_fn: Vec<bool>,
}

impl<'a> HookRuleCollector<'a> {
    fn add_finding(&mut self, start: usize, msg: String) {
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: msg,
        });
    }
}

impl<'a> Visit<'a> for HookRuleCollector<'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        if let oxc_ast::ast::Expression::Identifier(ident) = &expr.callee {
            let name = ident.name.as_str();
            if is_hook_call(name) {
                let in_loop = self.inside_loop.iter().any(|&v| v);
                let in_condition = self.inside_condition.iter().any(|&v| v);
                let in_nested_func = self.inside_nested_fn.len() > 1;

                if in_condition {
                    self.add_finding(
                        expr.span.start as usize,
                        format!("React hook `{name}` is called conditionally — move to top level"),
                    );
                }
                if in_loop {
                    self.add_finding(
                        expr.span.start as usize,
                        format!("React hook `{name}` is called inside a loop — move to top level"),
                    );
                }
                if in_nested_func {
                    self.add_finding(
                        expr.span.start as usize,
                        format!("React hook `{name}` is called inside a nested function"),
                    );
                }
            }
        }
    }

    fn visit_function(
        &mut self,
        func: &oxc_ast::ast::Function<'a>,
        _flags: oxc_syntax::scope::ScopeFlags,
    ) {
        self.inside_nested_fn.push(true);
        oxc_ast_visit::walk::walk_function(self, func, _flags);
        self.inside_nested_fn.pop();
    }

    fn visit_arrow_function_expression(
        &mut self,
        func: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        self.inside_nested_fn.push(true);
        oxc_ast_visit::walk::walk_arrow_function_expression(self, func);
        self.inside_nested_fn.pop();
    }

    fn visit_if_statement(&mut self, stmt: &oxc_ast::ast::IfStatement<'a>) {
        self.inside_condition.push(true);
        oxc_ast_visit::walk::walk_if_statement(self, stmt);
        self.inside_condition.pop();
    }

    fn visit_conditional_expression(&mut self, expr: &oxc_ast::ast::ConditionalExpression<'a>) {
        self.inside_condition.push(true);
        oxc_ast_visit::walk::walk_conditional_expression(self, expr);
        self.inside_condition.pop();
    }

    fn visit_logical_expression(&mut self, expr: &oxc_ast::ast::LogicalExpression<'a>) {
        if matches!(
            expr.operator,
            oxc_ast::ast::LogicalOperator::And | oxc_ast::ast::LogicalOperator::Or
        ) {
            self.inside_condition.push(true);
        }
        oxc_ast_visit::walk::walk_logical_expression(self, expr);
        self.inside_condition.pop();
    }

    fn visit_for_statement(&mut self, stmt: &oxc_ast::ast::ForStatement<'a>) {
        self.inside_loop.push(true);
        oxc_ast_visit::walk::walk_for_statement(self, stmt);
        self.inside_loop.pop();
    }

    fn visit_for_in_statement(&mut self, stmt: &oxc_ast::ast::ForInStatement<'a>) {
        self.inside_loop.push(true);
        oxc_ast_visit::walk::walk_for_in_statement(self, stmt);
        self.inside_loop.pop();
    }

    fn visit_for_of_statement(&mut self, stmt: &oxc_ast::ast::ForOfStatement<'a>) {
        self.inside_loop.push(true);
        oxc_ast_visit::walk::walk_for_of_statement(self, stmt);
        self.inside_loop.pop();
    }

    fn visit_while_statement(&mut self, stmt: &oxc_ast::ast::WhileStatement<'a>) {
        self.inside_loop.push(true);
        oxc_ast_visit::walk::walk_while_statement(self, stmt);
        self.inside_loop.pop();
    }

    fn visit_do_while_statement(&mut self, stmt: &oxc_ast::ast::DoWhileStatement<'a>) {
        self.inside_loop.push(true);
        oxc_ast_visit::walk::walk_do_while_statement(self, stmt);
        self.inside_loop.pop();
    }

    fn visit_switch_statement(&mut self, stmt: &oxc_ast::ast::SwitchStatement<'a>) {
        self.inside_condition.push(true);
        for case in &stmt.cases {
            oxc_ast_visit::walk::walk_switch_case(self, case);
        }
        self.inside_condition.pop();
    }
}

impl Rule for HookRules {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = HookRuleCollector {
            findings: Vec::new(),
            source: source_text,
            inside_loop: Vec::new(),
            inside_condition: Vec::new(),
            inside_nested_fn: Vec::new(),
        };
        collector.visit_program(program);
        collector.findings
    }
}
