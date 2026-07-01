use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoHardcodedSecrets;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-hardcoded-secrets",
    default_severity: Severity::Error,
    category: "security",
    description: "Flag potential API keys, tokens, passwords",
};

const SUSPICIOUS_NAMES: &[&str] = &[
    "api_key",
    "apiKey",
    "apikey",
    "api_secret",
    "apiSecret",
    "password",
    "passwd",
    "secret",
    "token",
    "auth_token",
    "access_token",
    "private_key",
    "privateKey",
];

const SUSPICIOUS_PATTERNS: &[&str] = &[
    "sk_live_", "sk_test_", "pk_live_", "pk_test_", "ghp_", "gho_", "ghu_", "ghs_", "AKIA",
    "xoxb-", "xoxp-",
];

impl Rule for NoHardcodedSecrets {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = SecretCollector {
            findings: Vec::new(),
            source: source_text,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct SecretCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for SecretCollector<'a> {
    fn visit_variable_declarator(&mut self, decl: &oxc_ast::ast::VariableDeclarator<'a>) {
        let name = if let oxc_ast::ast::BindingPatternKind::BindingIdentifier(id) = &decl.id.kind {
            id.name.as_str()
        } else {
            return;
        };

        let is_suspicious_name = SUSPICIOUS_NAMES.iter().any(|n| name.contains(n));

        if is_suspicious_name
            && let Some(init) = &decl.init
            && let oxc_ast::ast::Expression::StringLiteral(s) = init
        {
            let val = s.value.as_str();
            if val.len() > 8
                && val
                    .chars()
                    .any(|c| c.is_ascii_punctuation() || c.is_ascii_digit())
            {
                let start = decl.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: format!("Potential hardcoded secret in `{name}`"),
                });
            }
        }

        if let Some(init) = &decl.init
            && let oxc_ast::ast::Expression::StringLiteral(s) = init
        {
            let val = s.value.as_str();
            if SUSPICIOUS_PATTERNS.iter().any(|p| val.starts_with(p)) {
                let start = decl.span.start as usize;
                let line = self.source[..start].lines().count().max(1);
                let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                self.findings.push(RuleFinding {
                    line,
                    column: col + 1,
                    message: "Potential hardcoded secret (matches known pattern)".to_string(),
                });
            }
        }
    }
}
