use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_syntax::scope::ScopeFlags;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct ExplicitReturnType;

const RULE_META: RuleMeta = RuleMeta {
    id: "explicit-return-type",
    default_severity: Severity::Warning,
    category: "typescript",
    description: "Functions should have explicit return types",
};

impl Rule for ExplicitReturnType {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ReturnTypeCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ReturnTypeCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ReturnTypeCollector<'a> {
    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, _flags: ScopeFlags) {
        if func.return_type.is_none() && func.body.is_some() {
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
                message: format!("Function `{name}` is missing explicit return type"),
            });
        }
    }

    fn visit_arrow_function_expression(
        &mut self,
        func: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        if func.return_type.is_none() {
            let start = func.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Arrow function is missing explicit return type".to_string(),
            });
        }
    }
}
