use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct LazyLoadComponents;

const RULE_META: RuleMeta = RuleMeta {
    id: "lazy-load-components",
    default_severity: Severity::Warning,
    category: "performance",
    description: "Heavy components should be lazy-loaded with React.lazy",
};

impl Rule for LazyLoadComponents {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = LazyCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct LazyCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> LazyCollector<'a> {
    fn add_finding(&mut self, start: usize, msg: String) {
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: msg,
        });
    }
}

fn filename_starts_with_uppercase(path: &str) -> bool {
    path.rsplit('/')
        .next()
        .and_then(|name| name.chars().next())
        .is_some_and(|c| c.is_ascii_uppercase())
}

impl<'a> Visit<'a> for LazyCollector<'a> {
    fn visit_import_declaration(&mut self, decl: &oxc_ast::ast::ImportDeclaration<'a>) {
        let path = decl.source.value.as_str();

        // Check if any imported binding starts with uppercase (PascalCase component)
        let has_component_binding = decl.specifiers.as_ref().is_some_and(|specifiers| {
            specifiers.iter().any(|spec| {
                let local_name = match spec {
                    oxc_ast::ast::ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                        Some(s.local.name.as_str())
                    }
                    oxc_ast::ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                        Some(s.local.name.as_str())
                    }
                    oxc_ast::ast::ImportDeclarationSpecifier::ImportSpecifier(s) => {
                        Some(s.local.name.as_str())
                    }
                };
                local_name.is_some_and(|n| n.chars().next().is_some_and(|c| c.is_ascii_uppercase()))
            })
        });

        // Also check if the module path filename starts with uppercase
        let path_is_component = filename_starts_with_uppercase(path);

        if has_component_binding || path_is_component {
            self.add_finding(decl.span.start as usize,
                format!("Static import of component `{path}` — consider `React.lazy(() => import('{path}'))`"));
        }
    }
}
