use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct AssertIncludesMessage;

const RULE_META: RuleMeta = RuleMeta {
    id: "assert-includes-message",
    default_severity: Severity::Warning,
    category: "testing",
    description: "Assertions should include a descriptive message",
};

const ASSERT_METHODS: &[(&str, usize)] = &[
    ("assert", 1),
    ("assert.ok", 1),
    ("assert.strictEqual", 2),
    ("assert.deepEqual", 2),
    ("assert.equal", 2),
    ("assert.notStrictEqual", 2),
    ("assert.notEqual", 2),
    ("assert.deepStrictEqual", 2),
    ("assert.notDeepStrictEqual", 2),
    ("assert.throws", 1),
    ("assert.doesNotThrow", 1),
    ("assert.rejects", 1),
    ("assert.doesNotReject", 1),
];

impl Rule for AssertIncludesMessage {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = AssertCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct AssertCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> AssertCollector<'a> {
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

impl<'a> Visit<'a> for AssertCollector<'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        let callee_str = match &expr.callee {
            oxc_ast::ast::Expression::Identifier(ident) if ident.name.as_str() == "assert" => {
                "assert".to_string()
            }
            oxc_ast::ast::Expression::StaticMemberExpression(member) => {
                if let oxc_ast::ast::Expression::Identifier(obj) = &member.object {
                    if obj.name.as_str() == "assert" {
                        format!("assert.{}", member.property.name.as_str())
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            }
            _ => return,
        };

        let Some(&(_, required_args)) = ASSERT_METHODS.iter().find(|(name, _)| *name == callee_str)
        else {
            return;
        };

        if expr.arguments.len() <= required_args {
            let last_arg_has_msg = expr
                .arguments
                .last()
                .and_then(|arg| arg.as_expression())
                .is_some_and(|e| matches!(e, oxc_ast::ast::Expression::StringLiteral(_)));

            if !last_arg_has_msg {
                let method_name = if callee_str == "assert" {
                    "assert(condition)".to_string()
                } else {
                    format!("{callee_str}(…)")
                };
                self.push_finding(
                    expr.span.start as usize,
                    format!(
                        "`{method_name}` is missing a descriptive message — add a string argument explaining the assertion"
                    ),
                );
            }
        }
    }
}
