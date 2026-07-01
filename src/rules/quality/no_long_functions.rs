use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk;
use oxc_semantic::Semantic;
use oxc_syntax::scope::ScopeFlags;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoLongFunctions;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-long-functions",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Functions should not exceed 40 lines",
};

const MAX_LINES: usize = 40;

impl Rule for NoLongFunctions {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = LongFnCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct LongFnCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for LongFnCollector<'a> {
    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, _flags: ScopeFlags) {
        if let Some(body) = &func.body {
            let start_line = self.source[..body.span.start as usize].lines().count();
            let end_line = self.source[..body.span.end as usize].lines().count();
            let line_count = end_line - start_line;

            if line_count > MAX_LINES {
                let name = func
                    .id
                    .as_ref()
                    .map(|id| id.name.as_str())
                    .unwrap_or("anonymous");

                self.findings.push(RuleFinding {
                    line: start_line + 1,
                    column: 1,
                    message: format!(
                        "Function `{name}` is {line_count} lines long (max {MAX_LINES})"
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
        let start_line = self.source[..func.body.span.start as usize].lines().count();
        let end_line = self.source[..func.body.span.end as usize].lines().count();
        let line_count = end_line - start_line;

        if line_count > MAX_LINES {
            self.findings.push(RuleFinding {
                line: start_line + 1,
                column: 1,
                message: format!("Arrow function is {line_count} lines long (max {MAX_LINES})"),
            });
        }
        walk::walk_arrow_function_expression(self, func);
    }
}
