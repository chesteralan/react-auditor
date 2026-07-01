use oxc_ast::ast::Program;
use oxc_ast_visit::walk::walk_formal_parameters;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct MaxParams;

const RULE_META: RuleMeta = RuleMeta {
    id: "max-params",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Functions should have at most 3 parameters",
};

impl Rule for MaxParams {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ParamCollector { findings: Vec::new(), source: source_text };
        collector.visit_program(program);
        collector.findings
    }
}

struct ParamCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ParamCollector<'a> {
    fn visit_formal_parameters(&mut self, params: &oxc_ast::ast::FormalParameters<'a>) {
        if params.items.len() > 3 {
            let start = params.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: format!("Function has {} parameters, max allowed is 3", params.items.len()),
            });
        }
        walk_formal_parameters(self, params);
    }
}
