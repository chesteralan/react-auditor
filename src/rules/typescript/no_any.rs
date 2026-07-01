use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoAny;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-any",
    default_severity: Severity::Error,
    category: "typescript",
    description: "Avoid `any` — use `unknown` or proper types",
};

impl Rule for NoAny {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = AnyCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct AnyCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for AnyCollector<'a> {
    fn visit_ts_any_keyword(&mut self, ts: &oxc_ast::ast::TSAnyKeyword) {
        let start = ts.span.start as usize;
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: "Unexpected `any` — use `unknown` or a proper type".to_string(),
        });
    }
}
