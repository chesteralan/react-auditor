use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoHeavyComputationInRender;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-heavy-computation-in-render",
    default_severity: Severity::Warning,
    category: "performance",
    description: "Avoid heavy synchronous computation in render",
};

const HEAVY_METHODS: &[&str] = &[
    "sort",
    "filter",
    "map",
    "reduce",
    "forEach",
    "toSorted",
    "toReversed",
];

impl Rule for NoHeavyComputationInRender {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = HeavyComputeCollector {
            findings: Vec::new(),
            source: source_text,
            depth: 0,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct HeavyComputeCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    depth: usize,
}

impl<'a> HeavyComputeCollector<'a> {
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

impl<'a> Visit<'a> for HeavyComputeCollector<'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        if let Some(member) = expr.callee.as_member_expression()
            && let Some(name) = member.static_property_name()
            && HEAVY_METHODS.contains(&name)
            && self.depth > 0
        {
            self.add_finding(
                expr.span.start as usize,
                format!("`.{name}()` creates a new array on every render — memoize with useMemo"),
            );
        }
    }

    fn visit_function(
        &mut self,
        func: &oxc_ast::ast::Function<'a>,
        _flags: oxc_syntax::scope::ScopeFlags,
    ) {
        let was = self.depth;
        self.depth += 1;
        oxc_ast_visit::walk::walk_function(self, func, _flags);
        self.depth = was;
    }

    fn visit_arrow_function_expression(
        &mut self,
        func: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        let was = self.depth;
        self.depth += 1;
        oxc_ast_visit::walk::walk_arrow_function_expression(self, func);
        self.depth = was;
    }
}
