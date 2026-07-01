use oxc_ast::ast::{NumericLiteral, Program, VariableDeclarationKind};
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoMagicNumbers;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-magic-numbers",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Prefer named constants over magic numbers",
};

const ALLOWED: &[f64] = &[-1.0, 0.0, 1.0, 2.0, 100.0];

impl Rule for NoMagicNumbers {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = MagicCollector {
            findings: Vec::new(),
            source: source_text,
            in_const_decl: false,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct MagicCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    in_const_decl: bool,
}

impl<'a> Visit<'a> for MagicCollector<'a> {
    fn visit_variable_declarator(&mut self, decl: &oxc_ast::ast::VariableDeclarator<'a>) {
        let was_in_const = self.in_const_decl;
        self.in_const_decl = decl.kind == VariableDeclarationKind::Const;
        oxc_ast_visit::walk::walk_variable_declarator(self, decl);
        self.in_const_decl = was_in_const;
    }

    fn visit_numeric_literal(&mut self, lit: &NumericLiteral) {
        if !self.in_const_decl && !ALLOWED.contains(&lit.value) {
            let start = lit.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: format!("Magic number `{}` — use a named constant instead", lit.value),
            });
        }
    }
}
