use oxc_ast::ast::{AssignmentOperator, CallExpression, Expression, MemberExpression, Program};
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
    "Object",
    "Array",
    "Function",
    "String",
    "Number",
    "Boolean",
    "Symbol",
    "BigInt",
    "JSON",
    "Math",
    "Reflect",
    "RegExp",
    "Promise",
    "Map",
    "Set",
    "WeakMap",
    "WeakSet",
    "Date",
    "Error",
    "TypeError",
    "SyntaxError",
    "ReferenceError",
    "RangeError",
    "URIError",
    "console",
    "globalThis",
    "Intl",
    "Proxy",
    "ArrayBuffer",
    "SharedArrayBuffer",
    "DataView",
    "Atomics",
    "BigInt64Array",
    "BigUint64Array",
    "Float32Array",
    "Float64Array",
    "Int8Array",
    "Int16Array",
    "Int32Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "Uint16Array",
    "Uint32Array",
    "global",
    "process",
    "Buffer",
];

/// Returns `true` when the expression is provably non-null at runtime.
fn is_safe_expression(expr: &Expression) -> bool {
    match expr {
        // Literals are never null/undefined
        Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::TemplateLiteral(_)
        | Expression::ArrayExpression(_)
        | Expression::ObjectExpression(_)
        | Expression::FunctionExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::ClassExpression(_) => true,

        // `this` and `super` are never null
        Expression::ThisExpression(_) | Expression::Super(_) => true,

        // `new Foo()` always produces an object, never null
        Expression::NewExpression(_) => true,

        // `x!` — user already asserted non-null
        Expression::TSNonNullExpression(_) => true,

        // Well-known globals (Object, Array, Math, console, etc.)
        Expression::Identifier(ident) => SAFE_GLOBALS.contains(&ident.name.as_str()),

        // Unwrap type-only wrappers
        Expression::TSAsExpression(e) => is_safe_expression(&e.expression),
        Expression::TSSatisfiesExpression(e) => is_safe_expression(&e.expression),
        Expression::TSTypeAssertion(e) => is_safe_expression(&e.expression),
        Expression::TSInstantiationExpression(e) => is_safe_expression(&e.expression),
        Expression::ParenthesizedExpression(e) => is_safe_expression(&e.expression),

        // JSX elements are always objects
        Expression::JSXElement(_) | Expression::JSXFragment(_) => true,

        // PrivateIn and MetaProperty are always defined
        Expression::PrivateInExpression(_) | Expression::MetaProperty(_) => true,

        _ => false,
    }
}

impl<'a> Visit<'a> for NullCheckCollector<'a> {
    fn visit_member_expression(&mut self, expr: &MemberExpression<'a>) {
        let (object, optional) = match expr {
            MemberExpression::StaticMemberExpression(m) => (Some(&m.object), m.optional),
            MemberExpression::ComputedMemberExpression(m) => (Some(&m.object), m.optional),
            MemberExpression::PrivateFieldExpression(p) => (None, p.optional),
        };
        if let Some(obj) = object
            && is_safe_expression(obj)
        {
            return;
        }
        if optional {
            return;
        }
        match expr {
            MemberExpression::ComputedMemberExpression(_) => {
                self.add_finding(
                    expr.span().start as usize,
                    "Potential null access on computed member — consider optional chaining `?.[]`"
                        .to_string(),
                );
            }
            MemberExpression::StaticMemberExpression(_) => {
                self.add_finding(
                    expr.span().start as usize,
                    "Potential null access on property — consider optional chaining `?.`"
                        .to_string(),
                );
            }
            MemberExpression::PrivateFieldExpression(_) => {
                self.add_finding(
                    expr.span().start as usize,
                    "Potential null access on private field — consider optional chaining `?.`"
                        .to_string(),
                );
            }
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
            let (object, member_optional) = match member {
                MemberExpression::StaticMemberExpression(m) => (Some(&m.object), m.optional),
                MemberExpression::ComputedMemberExpression(m) => (Some(&m.object), m.optional),
                MemberExpression::PrivateFieldExpression(p) => (None, p.optional),
            };
            if let Some(obj) = object
                && is_safe_expression(obj)
            {
                return;
            }
            if !member_optional {
                self.add_finding(
                    expr.span.start as usize,
                    "Unsafe method call — consider optional chaining `?.()`".to_string(),
                );
            }
        }
    }
}
