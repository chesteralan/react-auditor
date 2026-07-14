use oxc_ast::ast::{Expression, Program};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoGlobalMutation;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-global-mutation",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Don't mutate built-in prototypes (Array, String, Object, etc.)",
};

const BUILTIN_PROTOTYPES: &[&str] = &[
    "Array",
    "String",
    "Number",
    "Boolean",
    "Object",
    "Function",
    "Symbol",
    "BigInt",
    "Date",
    "RegExp",
    "Map",
    "Set",
    "WeakMap",
    "WeakSet",
    "Promise",
    "Error",
    "TypeError",
    "RangeError",
    "SyntaxError",
    "ReferenceError",
];

impl Rule for NoGlobalMutation {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = GlobalMutationCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct GlobalMutationCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for GlobalMutationCollector<'a> {
    fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
        if is_prototype_mutation(&expr.left) {
            let start = expr.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Don't mutate built-in prototypes — extend via classes instead"
                    .to_string(),
            });
        }
        walk::walk_assignment_expression(self, expr);
    }
}

fn is_builtin_name(name: &str) -> bool {
    BUILTIN_PROTOTYPES.contains(&name)
}

fn is_prototype_mutation(target: &oxc_ast::ast::AssignmentTarget) -> bool {
    match target {
        oxc_ast::ast::AssignmentTarget::StaticMemberExpression(me) => {
            check_prototype_chain(me) || is_builtin_name(&me.property.name)
        }
        oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(me) => {
            if let Expression::StringLiteral(s) = &me.expression {
                is_builtin_name(&s.value)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn check_prototype_chain(me: &oxc_ast::ast::StaticMemberExpression) -> bool {
    if me.property.name == "prototype" {
        if let Expression::Identifier(id) = &me.object {
            return is_builtin_name(&id.name);
        }
        if let Expression::StaticMemberExpression(inner) = &me.object {
            return check_prototype_chain(inner);
        }
    }
    false
}
