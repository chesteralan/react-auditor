use oxc_ast::ast::{ImportOrExportKind, Program};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct ConsistentTypeImports;

const RULE_META: RuleMeta = RuleMeta {
    id: "consistent-type-imports",
    default_severity: Severity::Warning,
    category: "typescript",
    description: "Use `import type` for type-only imports",
};

impl Rule for ConsistentTypeImports {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = TypeImportCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct TypeImportCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for TypeImportCollector<'a> {
    fn visit_import_declaration(&mut self, decl: &oxc_ast::ast::ImportDeclaration<'a>) {
        if matches!(decl.import_kind, ImportOrExportKind::Type) {
            return;
        }
        let Some(specifiers) = &decl.specifiers else {
            return;
        };
        let all_type = specifiers.iter().all(|spec| {
            matches!(spec, oxc_ast::ast::ImportDeclarationSpecifier::ImportSpecifier(s) if matches!(s.import_kind, ImportOrExportKind::Type))
        });
        if all_type && !specifiers.is_empty() {
            let start = decl.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Use `import type` for type-only imports".to_string(),
            });
        }
    }
}
