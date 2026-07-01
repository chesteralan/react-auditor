use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoUnnecessaryMemo;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-unnecessary-memo",
    default_severity: Severity::Warning,
    category: "react",
    description: "Avoid useMemo/useCallback for trivial computations",
};

impl Rule for NoUnnecessaryMemo {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = UnnecessaryMemoCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct UnnecessaryMemoCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for UnnecessaryMemoCollector<'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        let name = if let oxc_ast::ast::Expression::Identifier(ident) = &expr.callee {
            Some(ident.name.as_str())
        } else if let Some(member) = expr.callee.as_member_expression() {
            member.static_property_name()
        } else {
            None
        };

        if let Some(name) = name
            && (name == "useMemo" || name == "useCallback")
            && expr.arguments.len() >= 2
            && let Some(last_arg) = expr.arguments.last()
            && let oxc_ast::ast::Argument::ArrayExpression(arr) = &last_arg
            && arr.elements.is_empty()
        {
            let start = expr.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: format!(
                    "`{name}` with empty deps array — value will never change, consider removing"
                ),
            });
        }
    }
}
