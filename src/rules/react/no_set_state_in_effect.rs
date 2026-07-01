use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoSetStateInEffect;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-set-state-in-effect",
    default_severity: Severity::Warning,
    category: "react",
    description: "Avoid calling setState in useEffect without conditions",
};

impl Rule for NoSetStateInEffect {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = SetStateEffectCollector {
            findings: Vec::new(),
            source: source_text,
            in_effect: false,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct SetStateEffectCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    in_effect: bool,
}

impl<'a> SetStateEffectCollector<'a> {
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

impl<'a> Visit<'a> for SetStateEffectCollector<'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        let name = if let oxc_ast::ast::Expression::Identifier(ident) = &expr.callee {
            ident.name.as_str()
        } else {
            return;
        };

        if name == "useEffect" || name == "useLayoutEffect" {
            self.in_effect = true;
            oxc_ast_visit::walk::walk_call_expression(self, expr);
            self.in_effect = false;
            return;
        }

        if self.in_effect
            && (name.starts_with("set")
                && name.len() > 3
                && name.as_bytes()[3].is_ascii_uppercase())
        {
            self.add_finding(expr.span.start as usize,
                format!("`{name}` called inside useEffect without dependencies — may cause infinite loop"));
        }

        if self.in_effect && name == "dispatch" {
            self.add_finding(
                expr.span.start as usize,
                "Store dispatch in useEffect without dependencies — may cause infinite loop"
                    .to_string(),
            );
        }
    }
}
