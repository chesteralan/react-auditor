use oxc_ast::ast::{Expression, Program};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct PreferObjectSpread;

const RULE_META: RuleMeta = RuleMeta {
    id: "prefer-object-spread",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Use object spread instead of Object.assign",
};

impl Rule for PreferObjectSpread {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ObjectSpreadCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ObjectSpreadCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ObjectSpreadCollector<'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        if is_object_assign_call(expr) {
            let start = expr.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let msg = if expr.arguments.len() == 2
                && matches!(&expr.arguments[0], oxc_ast::ast::Argument::ObjectExpression(obj) if obj.properties.is_empty())
            {
                "Use object spread `{...defaults, ...overrides}` instead of Object.assign with empty target".to_string()
            } else {
                "Use object spread instead of Object.assign".to_string()
            };
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: msg,
            });
        }
        walk::walk_call_expression(self, expr);
    }
}

fn is_object_assign_call(expr: &oxc_ast::ast::CallExpression) -> bool {
    if expr.arguments.len() < 2 {
        return false;
    }
    match &expr.callee {
        Expression::StaticMemberExpression(me) => {
            me.object.is_specific_id("Object") && me.property.name == "assign"
        }
        _ => false,
    }
}
