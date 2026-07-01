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
        let mut collector = LazyCollector { findings: Vec::new(), source: source_text };
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
        self.findings.push(RuleFinding { line, column: col + 1, message: msg });
    }
}

impl<'a> Visit<'a> for LazyCollector<'a> {
    fn visit_import_declaration(&mut self, decl: &oxc_ast::ast::ImportDeclaration<'a>) {
        let path = decl.source.value.as_str();
        let is_component_module = path.contains('/') && path.chars().last().is_some_and(|c| c.is_ascii_uppercase());
        if is_component_module {
            self.add_finding(decl.span.start as usize,
                format!("Static import of component `{path}` — consider `React.lazy(() => import('{path}'))`"));
        }
    }
}
