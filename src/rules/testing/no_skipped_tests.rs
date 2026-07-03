use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoSkippedTests;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-skipped-tests",
    default_severity: Severity::Warning,
    category: "testing",
    description: "No skipped or disabled tests committed",
};

impl Rule for NoSkippedTests {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = SkippedTestCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct SkippedTestCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> SkippedTestCollector<'a> {
    fn push_finding(&mut self, start: usize, msg: String) {
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: msg,
        });
    }
}

impl<'a> Visit<'a> for SkippedTestCollector<'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        let callee_str = match &expr.callee {
            oxc_ast::ast::Expression::Identifier(ident) => ident.name.as_str().to_string(),
            oxc_ast::ast::Expression::StaticMemberExpression(member) => {
                if let oxc_ast::ast::Expression::Identifier(obj) = &member.object {
                    format!("{}.{}", obj.name.as_str(), member.property.name.as_str())
                } else {
                    return;
                }
            }
            _ => return,
        };

        if callee_str == "it.skip"
            || callee_str == "describe.skip"
            || callee_str == "xit"
            || callee_str == "xdescribe"
        {
            self.push_finding(
                expr.span.start as usize,
                format!("Skipped test: `{callee_str}` — remove or implement before committing"),
            );
        }
    }
}
