use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoScriptTagInHead;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-script-tag-in-head",
    default_severity: Severity::Warning,
    category: "nextjs",
    description: "Use `next/script` instead of `<script>` inside `<Head>`",
};

impl Rule for NoScriptTagInHead {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ScriptHeadCollector {
            findings: Vec::new(),
            source: source_text,
            in_head: 0,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct ScriptHeadCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    in_head: usize,
}

impl<'a> ScriptHeadCollector<'a> {
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

impl<'a> Visit<'a> for ScriptHeadCollector<'a> {
    fn visit_jsx_element(&mut self, el: &oxc_ast::ast::JSXElement<'a>) {
        let is_head = match &el.opening_element.name {
            oxc_ast::ast::JSXElementName::Identifier(id) => id.name.as_str() == "head",
            oxc_ast::ast::JSXElementName::IdentifierReference(id) => id.name.as_str() == "Head",
            _ => false,
        };

        if is_head {
            self.in_head += 1;
        }

        oxc_ast_visit::walk::walk_jsx_element(self, el);

        if is_head {
            self.in_head = self.in_head.saturating_sub(1);
        }
    }

    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let is_script = matches!(
            &el.name,
            oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "script"
        );

        if self.in_head > 0 && is_script {
            self.add_finding(
                el.span.start as usize,
                "Use `next/script` (`<Script />`) instead of `<script>` inside `<Head>`"
                    .to_string(),
            );
        }
    }
}
