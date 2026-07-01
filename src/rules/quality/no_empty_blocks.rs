use oxc_ast::ast::{Program, Statement};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_statement;
use oxc_semantic::Semantic;

use crate::rules::{Fix, Rule, RuleFinding, RuleMeta, Severity};

pub struct NoEmptyBlocks;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-empty-blocks",
    default_severity: Severity::Warning,
    category: "quality",
    description: "No empty if/while/for/try/catch/finally blocks",
};

impl Rule for NoEmptyBlocks {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = EmptyBlockCollector {
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
        let start = crate::rules::line_col_to_offset(source_text, finding.line, finding.column)?;
        let after = &source_text[start..];
        let close = after.find('}')?;
        Some(Fix {
            start,
            end: start + close + 1,
            replacement: String::new(),
        })
    }
}

struct EmptyBlockCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for EmptyBlockCollector<'a> {
    fn visit_statement(&mut self, stmt: &Statement<'a>) {
        match stmt {
            Statement::IfStatement(if_stmt) => {
                if let Some(alt) = &if_stmt.alternate
                    && let Statement::BlockStatement(block) = alt
                    && block.body.is_empty()
                {
                    let start = block.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Empty else block".to_string(),
                    });
                }
                if let Statement::BlockStatement(block) = &if_stmt.consequent
                    && block.body.is_empty()
                {
                    let start = block.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Empty if block".to_string(),
                    });
                }
            }
            Statement::WhileStatement(while_stmt) => {
                if let Statement::BlockStatement(block) = &while_stmt.body
                    && block.body.is_empty()
                {
                    let start = block.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Empty while loop body".to_string(),
                    });
                }
            }
            Statement::ForStatement(for_stmt) => {
                if let Statement::BlockStatement(block) = &for_stmt.body
                    && block.body.is_empty()
                {
                    let start = block.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Empty for loop body".to_string(),
                    });
                }
            }
            Statement::TryStatement(try_stmt) => {
                if let Some(handler) = &try_stmt.handler
                    && handler.body.body.is_empty()
                {
                    let start = handler.body.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Empty catch block".to_string(),
                    });
                }
                if let Some(finalizer) = &try_stmt.finalizer
                    && finalizer.body.is_empty()
                {
                    let start = finalizer.span.start as usize;
                    let line = self.source[..start].lines().count().max(1);
                    let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    self.findings.push(RuleFinding {
                        line,
                        column: col + 1,
                        message: "Empty finally block".to_string(),
                    });
                }
            }
            _ => {}
        }

        walk_statement(self, stmt);
    }
}
