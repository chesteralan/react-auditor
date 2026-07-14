use oxc_ast::ast::{BindingPattern, Declaration, ExportDefaultDeclarationKind, Program, Statement};
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

    fn run(&self, program: &Program, semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut findings = Vec::new();
        let scoping = semantic.scoping();
        let exported_names = collect_exported_names(program);

        for symbol_id in scoping.symbol_ids() {
            let name = scoping.symbol_name(symbol_id);

            if name.starts_with('_') {
                continue;
            }

            // Skip TypeScript type-only declarations (interfaces, type aliases, type parameters)
            let flags = scoping.symbol_flags(symbol_id);
            if flags.is_interface() || flags.is_type_alias() || flags.is_type_parameter() {
                continue;
            }

            // Skip exported declarations (used by other modules)
            if exported_names.contains(&name) {
                continue;
            }

            let is_function = flags.contains(oxc_syntax::symbol::SymbolFlags::Function);

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
            }
        }

        findings
    }
}

fn collect_exported_names<'a>(program: &'a Program<'a>) -> Vec<&'a str> {
    fn push_names<'a>(pattern: &'a BindingPattern<'a>, names: &mut Vec<&'a str>) {
        match pattern {
            BindingPattern::BindingIdentifier(id) => {
                names.push(id.name.as_str());
            }
            BindingPattern::ObjectPattern(p) => {
                for prop in &p.properties {
                    push_names(&prop.value, names);
                }
            }
            BindingPattern::ArrayPattern(p) => {
                for elem in p.elements.iter().flatten() {
                    push_names(elem, names);
                }
            }
            BindingPattern::AssignmentPattern(p) => {
                push_names(&p.left, names);
            }
        }
    }
    let mut names = Vec::new();
    for stmt in &program.body {
        match stmt {
            Statement::ExportNamedDeclaration(export) => {
                if let Some(decl) = &export.declaration {
                    match decl {
                        Declaration::VariableDeclaration(var_decl) => {
                            for declarator in &var_decl.declarations {
                                push_names(&declarator.id, &mut names);
                            }
                        }
                        Declaration::FunctionDeclaration(func) => {
                            if let Some(id) = &func.id {
                                names.push(id.name.as_str());
                            }
                        }
                        Declaration::ClassDeclaration(class) => {
                            if let Some(id) = &class.id {
                                names.push(id.name.as_str());
                            }
                        }
                        _ => {}
                    }
                }
            }
            Statement::ExportDefaultDeclaration(decl) => match &decl.declaration {
                ExportDefaultDeclarationKind::FunctionDeclaration(f) => {
                    if let Some(id) = &f.id {
                        names.push(id.name.as_str());
                    }
                }
                ExportDefaultDeclarationKind::ClassDeclaration(c) => {
                    if let Some(id) = &c.id {
                        names.push(id.name.as_str());
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    names
}
