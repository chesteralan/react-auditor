use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoEmptyInterface;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-empty-interface",
    default_severity: Severity::Warning,
    category: "typescript",
    description: "No empty interfaces",
};

impl Rule for NoEmptyInterface {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = EmptyInterfaceCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct EmptyInterfaceCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for EmptyInterfaceCollector<'a> {
    fn visit_ts_interface_declaration(&mut self, decl: &oxc_ast::ast::TSInterfaceDeclaration<'a>) {
        if decl.body.body.is_empty() {
            let start = decl.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: format!(
                    "Interface `{}` is empty — remove or extend another type",
                    decl.id.name
                ),
            });
        }
    }
}
