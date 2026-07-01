use oxc_ast::ast::Program;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoUnusedVars;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-unused-vars",
    default_severity: Severity::Error,
    category: "quality",
    description: "No unused variables, imports, or parameters",
};

impl Rule for NoUnusedVars {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, _program: &Program, semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut findings = Vec::new();
        let scoping = semantic.scoping();

        for symbol_id in scoping.symbol_ids() {
            let name = scoping.symbol_name(symbol_id);
            let flags = scoping.symbol_flags(symbol_id);

            if name.starts_with('_') {
                continue;
            }

            let is_function = flags.contains(oxc_syntax::symbol::SymbolFlags::Function);
            let is_import = name == "import" || name == "default";

            let mut refs = semantic.symbol_references(symbol_id);
            let has_reads = refs.any(|r| r.flags().is_read());

            if !has_reads && !is_function {
                let span = scoping.symbol_span(symbol_id);
                let start = span.start as usize;
                let line = source_text[..start].lines().count().max(1);
                let col = start - source_text[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

                findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: format!("`{name}` is declared but never used"),
                });
                let _ = is_import;
            }
        }

        findings
    }
}
