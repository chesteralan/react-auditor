use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoSideEffectsInRender;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-side-effects-in-render",
    default_severity: Severity::Error,
    category: "react",
    description: "No side effects (subscriptions, mutations) during render",
};

impl Rule for NoSideEffectsInRender {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = SideEffectCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct SideEffectCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for SideEffectCollector<'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        if let oxc_ast::ast::Expression::Identifier(ident) = &expr.callee {
            match ident.name.as_str() {
                "addEventListener" | "removeEventListener"
                | "subscribe" | "unsubscribe"
                | "setTimeout" | "setInterval"
                | "MutationObserver" | "IntersectionObserver"
                | "fetch" | "XMLHttpRequest" => {
                    let start = expr.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: format!("Potential side effect `{}()` in render — move to useEffect", ident.name),
                    });
                }
                _ => {}
            }
        }
    }
}
