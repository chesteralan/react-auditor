use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct PreferFunctionComponents;

const RULE_META: RuleMeta = RuleMeta {
    id: "prefer-function-components",
    default_severity: Severity::Warning,
    category: "react",
    description: "Prefer function components over class components",
};

impl Rule for PreferFunctionComponents {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ClassComponentCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct ClassComponentCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ClassComponentCollector<'a> {
    fn visit_class(&mut self, class: &oxc_ast::ast::Class<'a>) {
        let extends_react = class.super_class.as_ref().is_some_and(|expr| {
            if let oxc_ast::ast::Expression::Identifier(ident) = expr {
                let name = ident.name.as_str();
                name == "Component" || name == "PureComponent"
            } else if let Some(member) = expr.as_member_expression() {
                member.static_property_name().is_some_and(|n| n == "Component" || n == "PureComponent")
            } else {
                false
            }
        });

        if extends_react
            && let Some(id) = &class.id {
                let start = id.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: format!("`{}` extends Component — prefer a function component", id.name),
                });
            }
    }
}
