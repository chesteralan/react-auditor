use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoDirectMutation;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-direct-mutation",
    default_severity: Severity::Warning,
    category: "react",
    description: "Avoid direct mutation of state or props — use setState or dispatch",
};

impl Rule for NoDirectMutation {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = DirectMutationCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct DirectMutationCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> DirectMutationCollector<'a> {
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

impl<'a> Visit<'a> for DirectMutationCollector<'a> {
    fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
        if let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(ident) = &expr.left {
            let name = ident.name.as_str();
            if name == "props" || name == "state" {
                self.add_finding(
                    expr.span.start as usize,
                    format!(
                        "Direct mutation of `{name}` — use setState or immutable patterns instead"
                    ),
                );
            }
        }
        if let oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(member) = &expr.left
            && let oxc_ast::ast::Expression::Identifier(ident) = &member.object
        {
            let name = ident.name.as_str();
            if name == "props" || name == "state" {
                self.add_finding(
                    expr.span.start as usize,
                    format!(
                        "Direct mutation of `{name}` — use setState or immutable patterns instead"
                    ),
                );
            }
        }
        if let oxc_ast::ast::AssignmentTarget::StaticMemberExpression(member) = &expr.left
            && let oxc_ast::ast::Expression::StaticMemberExpression(inner) = &member.object
            && (inner.property.name.as_str() == "state" || inner.property.name.as_str() == "props")
            && let oxc_ast::ast::Expression::ThisExpression(_) = &inner.object
        {
            self.add_finding(
                expr.span.start as usize,
                format!(
                    "Direct mutation of `this.{}.{}` — use setState or immutable patterns",
                    inner.property.name, member.property.name
                ),
            );
        }
    }
}
