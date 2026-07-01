use oxc_ast::ast::{Program, VariableDeclarationKind};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_variable_declaration;
use oxc_semantic::Semantic;

use crate::rules::{Fix, Rule, RuleFinding, RuleMeta, Severity};

pub struct NoVar;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-var",
    default_severity: Severity::Error,
    category: "quality",
    description: "Use const or let instead of var",
};

impl Rule for NoVar {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = VarCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }

    fn has_fix(&self) -> bool {
        true
    }

    fn fix(&self, finding: &RuleFinding, source_text: &str) -> Option<Fix> {
        let offset = crate::rules::line_col_to_offset(source_text, finding.line, finding.column)?;
        let after = &source_text[offset..];
        let var_len = after.find([' ', '\t', '\n', ';'])?;
        if after.starts_with("var ") {
            Some(Fix {
                start: offset,
                end: offset + var_len,
                replacement: "const ".to_string(),
            })
        } else if after.starts_with("var\t") {
            Some(Fix {
                start: offset,
                end: offset + var_len,
                replacement: "const\t".to_string(),
            })
        } else {
            None
        }
    }
}

struct VarCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for VarCollector<'a> {
    fn visit_variable_declaration(&mut self, decl: &oxc_ast::ast::VariableDeclaration<'a>) {
        if decl.kind == VariableDeclarationKind::Var {
            let start = decl.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);

            let names: Vec<String> = decl
                .declarations
                .iter()
                .filter_map(|d| {
                    if let oxc_ast::ast::BindingPattern::BindingIdentifier(id) = &d.id {
                        Some(id.name.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            let vars = names.join(", ");
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: format!("Use const or let instead of var: {vars}"),
            });
        }
        walk_variable_declaration(self, decl);
    }
}
