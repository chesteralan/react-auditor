use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct EffectDepsComplete;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-missing-deps",
    default_severity: Severity::Warning,
    category: "react",
    description: "useEffect/useMemo/useCallback should have a dependency array",
};

fn is_hook_with_deps(name: &str) -> bool {
    matches!(
        name,
        "useEffect" | "useMemo" | "useCallback" | "useLayoutEffect" | "useInsertionEffect"
    )
}

impl Rule for EffectDepsComplete {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = DepsCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct DepsCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> DepsCollector<'a> {
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

impl<'a> Visit<'a> for DepsCollector<'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        let name = if let oxc_ast::ast::Expression::Identifier(ident) = &expr.callee {
            ident.name.as_str()
        } else {
            return;
        };

        if !is_hook_with_deps(name) {
            return;
        }

        if expr.arguments.len() < 2 {
            self.add_finding(
                expr.span.start as usize,
                format!("`{name}` is missing a dependency array"),
            );
            return;
        }

        if let Some(oxc_ast::ast::Argument::ArrayExpression(arr)) = expr.arguments.last()
            && arr.elements.is_empty()
            && (name == "useEffect" || name == "useLayoutEffect" || name == "useInsertionEffect")
        {
            self.add_finding(
                expr.span.start as usize,
                format!("`{name}` has empty deps array — likely a bug"),
            );
        }
    }
}
