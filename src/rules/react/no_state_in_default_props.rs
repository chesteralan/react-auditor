use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoStateInDefaultProps;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-state-in-default-props",
    default_severity: Severity::Warning,
    category: "react",
    description: "Don't derive default props from state",
};

impl Rule for NoStateInDefaultProps {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = StateInDefaultPropsCollector {
            findings: Vec::new(),
            source: source_text,
            in_default_props: false,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct StateInDefaultPropsCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    in_default_props: bool,
}

impl<'a> StateInDefaultPropsCollector<'a> {
    fn push_finding(&mut self, start: usize) {
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: "Default props should not reference component state".to_string(),
        });
    }
}

impl<'a> Visit<'a> for StateInDefaultPropsCollector<'a> {
    fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
        let is_default_props = matches!(
            &expr.left,
            oxc_ast::ast::AssignmentTarget::StaticMemberExpression(member)
                if member.property.name.as_str() == "defaultProps"
        );
        if is_default_props {
            self.in_default_props = true;
            oxc_ast_visit::walk::walk_assignment_expression(self, expr);
            self.in_default_props = false;
            return;
        }
        if !self.in_default_props {
            oxc_ast_visit::walk::walk_assignment_expression(self, expr);
        }
    }

    fn visit_this_expression(&mut self, expr: &oxc_ast::ast::ThisExpression) {
        if self.in_default_props {
            self.push_finding(expr.span.start as usize);
        }
    }
}
