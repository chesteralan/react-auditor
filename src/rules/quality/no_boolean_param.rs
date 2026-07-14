use oxc_ast::ast::Program;
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::Semantic;
use oxc_syntax::scope::ScopeFlags;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoBooleanParam;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-boolean-param",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Avoid boolean parameters — split into multiple functions instead",
};

impl Rule for NoBooleanParam {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = BoolParamCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct BoolParamCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for BoolParamCollector<'a> {
    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, _flags: ScopeFlags) {
        for param in &func.params.items {
            if let Some(type_ann) = &param.type_annotation
                && matches!(
                    type_ann.type_annotation,
                    oxc_ast::ast::TSType::TSBooleanKeyword(_)
                )
            {
                let name = match &param.pattern {
                    oxc_ast::ast::BindingPattern::BindingIdentifier(id) => id.name.as_str(),
                    _ => "param",
                };
                let start = param.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: format!(
                        "Boolean parameter `{name}` — split function instead of using a flag"
                    ),
                });
            }
        }
        walk::walk_function(self, func, _flags);
    }

    fn visit_arrow_function_expression(
        &mut self,
        func: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        for param in &func.params.items {
            if let Some(type_ann) = &param.type_annotation
                && matches!(
                    type_ann.type_annotation,
                    oxc_ast::ast::TSType::TSBooleanKeyword(_)
                )
            {
                let start = param.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: "Boolean parameter — split arrow function instead of using a flag"
                        .to_string(),
                });
            }
        }
        walk::walk_arrow_function_expression(self, func);
    }
}
