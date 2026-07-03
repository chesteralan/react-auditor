use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Fix, Rule, RuleFinding, RuleMeta, Severity};

pub struct PreferFragments;

const RULE_META: RuleMeta = RuleMeta {
    id: "prefer-fragments",
    default_severity: Severity::Warning,
    category: "performance",
    description: "Use `<></>` over unnecessary wrapper divs",
};

impl Rule for PreferFragments {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = FragmentCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }

    fn has_fix(&self) -> bool {
        true
    }

    fn fix(&self, finding: &RuleFinding, source_text: &str) -> Option<Fix> {
        let offset = crate::rules::line_col_to_offset(source_text, finding.line, finding.column)?;
        let after = &source_text[offset..];
        if !after.starts_with("<div") {
            return None;
        }
        let opening_close = after.find('>')?;
        let content_start = offset + opening_close + 1;

        let closing_tag_start = find_matching_closing_div(source_text, content_start)?;
        let closing_tag_end = closing_tag_start + 6;

        let inner = &source_text[content_start..closing_tag_start];

        Some(Fix {
            start: offset,
            end: closing_tag_end,
            replacement: format!("<>{inner}</>"),
        })
    }
}

fn find_matching_closing_div(source: &str, start: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut depth = 1u32;
    let mut i = start;
    while i + 5 < bytes.len() {
        if bytes[i] == b'<' && bytes[i + 1] == b'/' && bytes[i + 2] == b'd'
            && bytes[i + 3] == b'i' && bytes[i + 4] == b'v' && bytes[i + 5] == b'>'
        {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
            i += 6;
        } else if bytes[i] == b'<' && bytes[i + 1] != b'/'
            && i + 4 < bytes.len() && bytes[i + 1] == b'd'
            && bytes[i + 2] == b'i' && bytes[i + 3] == b'v'
            && (bytes[i + 4] == b'>' || bytes[i + 4] == b' ' || bytes[i + 4] == b'\t' || bytes[i + 4] == b'\n')
        {
            depth += 1;
            let tag_close = source[i..].find('>')?;
            i += tag_close + 1;
        } else {
            i += 1;
        }
    }
    None
}

struct FragmentCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for FragmentCollector<'a> {
    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        let is_div = matches!(&el.name, oxc_ast::ast::JSXElementName::Identifier(id) if id.name.as_str() == "div");
        if !is_div {
            return;
        }
        let has_class = el.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr {
                matches!(&a.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "className" || id.name.as_str() == "class")
            } else {
                false
            }
        });
        if has_class {
            return;
        }
        let has_style = el.attributes.iter().any(|attr| {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(a) = attr {
                matches!(&a.name, oxc_ast::ast::JSXAttributeName::Identifier(id) if id.name.as_str() == "style")
            } else {
                false
            }
        });
        if !has_class && !has_style {
            let start = el.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: "Unnecessary `<div>` wrapper — use `<></>` Fragment instead".to_string(),
            });
        }
    }
}
