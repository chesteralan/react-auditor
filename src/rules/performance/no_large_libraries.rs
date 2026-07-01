use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

const HEAVY_LIBS: &[(&str, &str)] = &[
    ("moment", "Use `date-fns` or `dayjs` instead of `moment`"),
    (
        "lodash",
        "Use native array/object methods or `rambda` instead of `lodash`",
    ),
    (
        "underscore",
        "Use native array/object methods instead of `underscore`",
    ),
    (
        "jquery",
        "Avoid jQuery in React projects — use React refs and state",
    ),
    (
        "axios",
        "Prefer `fetch` or `ky` over `axios` for HTTP requests",
    ),
];

pub struct NoLargeLibraries;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-large-libraries",
    default_severity: Severity::Warning,
    category: "performance",
    description: "Avoid importing large libraries when lighter alternatives exist",
};

impl Rule for NoLargeLibraries {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = LargeLibCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct LargeLibCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> LargeLibCollector<'a> {
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

impl<'a> Visit<'a> for LargeLibCollector<'a> {
    fn visit_import_declaration(&mut self, decl: &oxc_ast::ast::ImportDeclaration<'a>) {
        let path = decl.source.value.as_str();
        for (lib_name, suggestion) in HEAVY_LIBS {
            if path == *lib_name || path.starts_with(&format!("{lib_name}/")) {
                self.add_finding(
                    decl.span.start as usize,
                    format!("Avoid importing `{path}`. {suggestion}"),
                );
            }
        }
    }
}
