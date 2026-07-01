use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoMultipleRenderMethods;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-multiple-render-methods",
    default_severity: Severity::Warning,
    category: "react",
    description: "Component should not have multiple render methods",
};

impl Rule for NoMultipleRenderMethods {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = MultipleRenderCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct MultipleRenderCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for MultipleRenderCollector<'a> {
    fn visit_class(&mut self, class: &oxc_ast::ast::Class<'a>) {
        let render_methods: Vec<_> = class.body.body.iter().filter(|m| {
            if let oxc_ast::ast::ClassElement::MethodDefinition(method) = m {
                method.key.static_name().is_some_and(|n| n == "render")
            } else {
                false
            }
        }).collect();

        if render_methods.len() > 1 {
            let start = class.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: format!("Component has {} render methods — split into separate components", render_methods.len()),
            });
        }
    }
}
