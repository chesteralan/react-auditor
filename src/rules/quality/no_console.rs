use oxc_ast::ast::{CallExpression, Expression, Program};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_call_expression;
use oxc_semantic::Semantic;

use crate::rules::{Fix, Rule, RuleFinding, RuleMeta, Severity};

pub struct NoConsole;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-console",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Avoid console.log in production code",
};

impl Rule for NoConsole {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ConsoleCallCollector {
            calls: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.calls
    }

    fn has_fix(&self) -> bool {
        true
    }

    fn fix(&self, finding: &RuleFinding, source_text: &str) -> Option<Fix> {
        let start = crate::rules::line_col_to_offset(source_text, finding.line, finding.column)?;
        let after = &source_text[start..];
        let mut depth = 0;
        let mut end = 0;
        for (i, ch) in after.char_indices() {
            if ch == '(' {
                depth += 1;
            } else if ch == ')' {
                depth -= 1;
                if depth == 0 {
                    end = start + i + 1;
                    break;
                }
            }
        }
        if depth != 0 || end == 0 {
            return None;
        }
        let after_fix = &source_text[end..];
        if after_fix.starts_with(';') {
            end += 1;
        }
        Some(Fix {
            start,
            end,
            replacement: String::new(),
        })
    }
}

struct ConsoleCallCollector<'a> {
    calls: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ConsoleCallCollector<'a> {
    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        if let Expression::StaticMemberExpression(member) = &expr.callee
            && let Expression::Identifier(ident) = &member.object
            && ident.name.as_str() == "console"
        {
            let start = expr.span.start as usize;
            let end = expr.span.end as usize;

            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let snippet = &self.source[start..end.min(self.source.len())];

            self.calls.push(RuleFinding {
                line,
                column: col + 1,
                message: format!("Unexpected console statement: {snippet}"),
            });
        }

        walk_call_expression(self, expr);
    }
}
