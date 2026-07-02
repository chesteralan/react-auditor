use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoForwardRef;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-forward-ref",
    default_severity: Severity::Warning,
    category: "react",
    description: "forwardRef is deprecated in React 19; use ref as a prop instead",
};

impl Rule for NoForwardRef {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ForwardRefCollector {
            findings: Vec::new(),
            source: source_text,
            imports_react: false,
            has_forward_ref_import: false,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ForwardRefCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    imports_react: bool,
    has_forward_ref_import: bool,
}

impl<'a> Visit<'a> for ForwardRefCollector<'a> {
    fn visit_import_declaration(&mut self, decl: &oxc_ast::ast::ImportDeclaration<'a>) {
        if decl.source.value.as_str() == "react" {
            self.imports_react = true;
            if let Some(specifiers) = &decl.specifiers {
                for spec in specifiers.iter() {
                    if let oxc_ast::ast::ImportDeclarationSpecifier::ImportSpecifier(s) = spec {
                        if s.imported.name().as_str() == "forwardRef" {
                            self.has_forward_ref_import = true;
                        }
                    }
                }
            }
        }
    }

    fn visit_call_expression(&mut self, call: &oxc_ast::ast::CallExpression<'a>) {
        let is_forward_ref = match &call.callee {
            oxc_ast::ast::Expression::Identifier(ident) => {
                self.has_forward_ref_import && ident.name.as_str() == "forwardRef"
            }
            oxc_ast::ast::Expression::StaticMemberExpression(member) => {
                let is_react = matches!(
                    &member.object,
                    oxc_ast::ast::Expression::Identifier(id) if id.name.as_str() == "React"
                );
                self.imports_react && is_react && member.property.name.as_str() == "forwardRef"
            }
            _ => false,
        };
        if is_forward_ref {
            let start = call.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "forwardRef is deprecated in React 19; pass ref as a regular prop instead"
                    .to_string(),
            });
        }
    }
}
