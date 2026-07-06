use oxc_ast::ast::{AssignmentOperator, CallExpression, MemberExpression, Program};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;
use oxc_span::GetSpan;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct StrictNullChecks;

const RULE_META: RuleMeta = RuleMeta {
    id: "strict-null-checks",
    default_severity: Severity::Warning,
    category: "typescript",
    description: "Prefer optional chaining and null checks on nullable values",
};

impl Rule for StrictNullChecks {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = NullCheckCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct NullCheckCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> NullCheckCollector<'a> {
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

/// Well-known global identifiers that are never null or undefined.
const SAFE_GLOBALS: &[&str] = &[
    "Object", "Array", "Function", "String", "Number", "Boolean", "Symbol", "BigInt",
    "JSON", "Math", "Reflect", "RegExp", "Promise", "Map", "Set", "WeakMap", "WeakSet",
    "Date", "Error", "TypeError", "SyntaxError", "ReferenceError", "RangeError", "URIError",
    "console", "globalThis", "Intl", "Proxy", "ArrayBuffer", "SharedArrayBuffer", "DataView",
    "Atomics", "BigInt64Array", "BigUint64Array", "Float32Array", "Float64Array",
    "Int8Array", "Int16Array", "Int32Array", "Uint8Array", "Uint8ClampedArray",
    "Uint16Array", "Uint32Array", "global", "process", "Buffer",
];

fn is_safe_global(expr: &oxc_ast::ast::Expression) -> bool {
    if let oxc_ast::ast::Expression::Identifier(ident) = expr {
        SAFE_GLOBALS.contains(&ident.name.as_str())
    } else {
        false
    }
}

impl<'a> Visit<'a> for NullCheckCollector<'a> {
    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        let is_safe = match expr {
            MemberExpression::StaticMemberExpression(m) => is_safe_global(&m.object),
            MemberExpression::ComputedMemberExpression(m) => is_safe_global(&m.object),
            MemberExpression::PrivateFieldExpression(_) => false,
        };
        if is_safe {
            return;
        }

        if let MemberExpression::ComputedMemberExpression(computed) = expr
            && !computed.optional
        {
            self.add_finding(
                expr.span().start as usize,
                "Potential null access on computed member — consider optional chaining `?.[]`"
                    .to_string(),
            );
        }
        if let MemberExpression::StaticMemberExpression(static_member) = expr
            && !static_member.optional
        {
            self.add_finding(
                expr.span().start as usize,
                "Potential null access on property — consider optional chaining `?.`".to_string(),
            );
        }
    }

    fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
        if matches!(expr.operator, AssignmentOperator::Assign)
            && let oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(_member) = &expr.left
        {
            self.add_finding(
                expr.span.start as usize,
                "Unsafe property write via computed access — ensure value is not null/undefined"
                    .to_string(),
            );
        }
    }

    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        if expr.optional {
            return;
        }
        if let Some(member) = expr.callee.as_member_expression() {
            let is_safe = match member {
                MemberExpression::StaticMemberExpression(m) => is_safe_global(&m.object),
                MemberExpression::ComputedMemberExpression(m) => is_safe_global(&m.object),
                MemberExpression::PrivateFieldExpression(_) => false,
            };
            if is_safe {
                return;
            }

            let optional = match member {
                MemberExpression::ComputedMemberExpression(c) => c.optional,
                MemberExpression::StaticMemberExpression(s) => s.optional,
                MemberExpression::PrivateFieldExpression(_) => false,
            };
            if !optional {
                self.add_finding(
                    expr.span.start as usize,
                    "Unsafe method call — consider optional chaining `?.()`".to_string(),
                );
            }
        }
    }
}
