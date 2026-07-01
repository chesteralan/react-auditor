use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct HeadingLevels;

const RULE_META: RuleMeta = RuleMeta {
    id: "heading-levels",
    default_severity: Severity::Warning,
    category: "accessibility",
    description: "Heading levels should not be skipped",
};

impl Rule for HeadingLevels {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = HeadingCollector {
            findings: Vec::new(),
            source: source_text,
            prev_level: 0,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct HeadingCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    prev_level: u8,
}

impl<'a> HeadingCollector<'a> {
    fn add_finding(&mut self, start: usize, msg: String) {
        let line = self.source[..start].lines().count().max(1);
        let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        self.findings.push(RuleFinding {
            line,
            column: col + 1,
            message: msg,
        });
    }

    fn parse_heading(el: &oxc_ast::ast::JSXOpeningElement) -> Option<u8> {
        if let oxc_ast::ast::JSXElementName::Identifier(id) = &el.name {
            let name = id.name.as_str();
            if name.len() == 2 && name.starts_with('h') {
                let n = name.as_bytes()[1].wrapping_sub(b'0');
                if (1..=6).contains(&n) {
                    return Some(n);
                }
            }
        }
        None
    }
}

impl<'a> Visit<'a> for HeadingCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let Some(level) = Self::parse_heading(el) else {
            return;
        };

        if self.prev_level > 0 && level > self.prev_level + 1 {
            self.add_finding(
                el.span.start as usize,
                format!(
                    "Heading level skipped: h{} followed by h{}",
                    self.prev_level, level
                ),
            );
        }
        self.prev_level = level;
    }
}
