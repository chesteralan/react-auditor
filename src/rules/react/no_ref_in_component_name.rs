use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoRefInComponentName;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-ref-in-component-name",
    default_severity: Severity::Warning,
    category: "react",
    description: "Component names should not contain 'Ref'",
};

impl Rule for NoRefInComponentName {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = RefNameCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct RefNameCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for RefNameCollector<'a> {
    fn visit_variable_declarator(&mut self, decl: &oxc_ast::ast::VariableDeclarator<'a>) {
        if let oxc_ast::ast::BindingPatternKind::BindingIdentifier(ident) = &decl.id.kind {
            let name = ident.name.as_str();
            if is_pascal_case(name) && (name.contains("Ref") || name.contains("ref")) {
                let start = ident.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: format!("Component name `{name}` contains 'Ref' — consider renaming"),
                });
            }
        }
    }

    fn visit_function(
        &mut self,
        func: &oxc_ast::ast::Function<'a>,
        _flags: oxc_syntax::scope::ScopeFlags,
    ) {
        if let Some(ident) = &func.id {
            let name = ident.name.as_str();
            if is_pascal_case(name) && (name.contains("Ref") || name.contains("ref")) {
                let start = ident.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: format!("Component name `{name}` contains 'Ref' — consider renaming"),
                });
            }
        }
    }
}

fn is_pascal_case(s: &str) -> bool {
    let first = s.chars().next();
    matches!(first, Some(c) if c.is_ascii_uppercase())
}
