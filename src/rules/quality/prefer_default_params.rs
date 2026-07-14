use oxc_ast::ast::{Expression, LogicalOperator, Program, Statement};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::Semantic;
use oxc_syntax::scope::ScopeFlags;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct PreferDefaultParams;

const RULE_META: RuleMeta = RuleMeta {
    id: "prefer-default-params",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Use default parameters instead of short-circuiting or conditionals",
};

impl Rule for PreferDefaultParams {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = DefaultParamCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct DefaultParamCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for DefaultParamCollector<'a> {
    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, _flags: ScopeFlags) {
        check_for_default_param_patterns(
            func.body.as_ref().map(|b| b.as_ref()),
            &mut self.findings,
            self.source,
        );
        walk::walk_function(self, func, _flags);
    }

    fn visit_arrow_function_expression(
        &mut self,
        func: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        check_for_default_param_patterns(Some(func.body.as_ref()), &mut self.findings, self.source);
        walk::walk_arrow_function_expression(self, func);
    }
}

fn check_for_default_param_patterns(
    body: Option<&oxc_ast::ast::FunctionBody>,
    findings: &mut Vec<RuleFinding>,
    source: &str,
) {
    let body = match body {
        Some(b) => b,
        None => return,
    };
    for stmt in &body.statements {
        let Statement::ExpressionStatement(es) = stmt else {
            continue;
        };
        if let Expression::AssignmentExpression(assign) = &es.expression
            && matches!(assign.operator, oxc_ast::ast::AssignmentOperator::Assign)
        {
            let name = match &assign.left {
                oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) => id.name.as_str(),
                _ => continue,
            };
            match &assign.right {
                Expression::LogicalExpression(logical)
                    if logical.operator == LogicalOperator::Or =>
                {
                    if let Expression::Identifier(id) = &logical.left
                        && id.name == name
                    {
                        let start = assign.span.start as usize;
                        let line = source[..start].lines().count().max(1);
                        let col = start - source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                        findings.push(RuleFinding {
                            line,
                            column: col + 1,
                            message: format!(
                                "Use a default parameter for `{name}` instead of `||`"
                            ),
                        });
                    }
                }
                Expression::ConditionalExpression(cond) => {
                    if let Expression::BinaryExpression(bin) = &cond.test
                        && matches!(
                            bin.operator,
                            oxc_ast::ast::BinaryOperator::StrictInequality
                                | oxc_ast::ast::BinaryOperator::Inequality
                        )
                        && bin.left.is_specific_id(name)
                        && (matches!(&bin.right, Expression::Identifier(id) if id.name == "undefined")
                            || matches!(&bin.right, Expression::NullLiteral(_)))
                    {
                        let start = assign.span.start as usize;
                        let line = source[..start].lines().count().max(1);
                        let col = start - source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                        findings.push(RuleFinding {
                            line,
                            column: col + 1,
                            message: format!(
                                "Use a default parameter for `{name}` instead of conditional"
                            ),
                        });
                    }
                }
                _ => {}
            }
        }
    }
}
