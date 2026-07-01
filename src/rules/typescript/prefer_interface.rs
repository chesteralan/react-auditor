use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct PreferInterface;

const RULE_META: RuleMeta = RuleMeta {
    id: "prefer-interface",
    default_severity: Severity::Warning,
    category: "typescript",
    description: "Prefer `interface` over `type` for object types",
};

impl Rule for PreferInterface {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = TypeAliasCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct TypeAliasCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for TypeAliasCollector<'a> {
    fn visit_ts_type_alias_declaration(&mut self, decl: &oxc_ast::ast::TSTypeAliasDeclaration<'a>) {
        let is_object = matches!(
            &decl.type_annotation,
            oxc_ast::ast::TSType::TSTypeLiteral(_)
        );
        if is_object {
            let start = decl.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: format!(
                    "`type {} = {{...}}` — use `interface {} {{...}}` instead",
                    decl.id.name, decl.id.name
                ),
            });
        }
    }
}
