use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoEval;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-eval",
    default_severity: Severity::Error,
    category: "security",
    description: "No eval, Function(), or setTimeout with string args",
};

impl Rule for NoEval {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = EvalCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct EvalCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for EvalCollector<'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        if let oxc_ast::ast::Expression::Identifier(ident) = &expr.callee {
            let name = ident.name.as_str();
            if name == "eval" {
                let start = expr.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: "Unexpected `eval()` — security risk".to_string(),
                });
            }
        } else if let Some(member) = expr.callee.as_member_expression() {
            let name = member.static_property_name().unwrap_or("");
            let is_dynamic = name == "Function" || name == "constructor" || name == "setTimeout" || name == "setInterval";
            if is_dynamic {
                let obj = member.object();
                if let oxc_ast::ast::Expression::Identifier(ident) = obj
                    && (ident.name.as_str() == "window" || ident.name.as_str() == "globalThis") {
                        let start = expr.span.start as usize;
                        let line = self.source[..start].lines().count().max(1);
                        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                        self.findings.push(RuleFinding {
                            line,
                            column: col + 1,
                            message: format!("Unexpected dynamic code execution via `{}`", name),
                        });
                    }
            }
        }
    }
}
