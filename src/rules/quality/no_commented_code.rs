use oxc_ast::ast::Program;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoCommentedCode;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-commented-code",
    default_severity: Severity::Warning,
    category: "quality",
    description: "Remove dead commented-out code",
};

const CODE_KEYWORDS: &[&str] = &[
    "function ", "const ", "let ", "var ", "if ", "else ", "for ", "while ",
    "return ", "import ", "export ", "class ", "new ", "try ", "catch ",
    "=>", "console.", "document.", "require(",
];

impl Rule for NoCommentedCode {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, _program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut findings = Vec::new();
        let lines: Vec<&str> = source_text.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            let trimmed = lines[i].trim();
            if trimmed.starts_with("//") {
                let comment_body = &trimmed[2..].trim();
                if looks_like_code(comment_body) {
                    findings.push(RuleFinding {
                        line: i + 1,
                        column: 1,
                        message: format!("Commented-out code: {comment_body}"),
                    });
                }
            } else if trimmed.starts_with("/*") {
                // Collect multi-line comment
                let mut comment_lines = vec![];
                let start_line = i;
                while i < lines.len() {
                    comment_lines.push(lines[i]);
                    if lines[i].contains("*/") {
                        break;
                    }
                    i += 1;
                }
                let full_comment = comment_lines.join("\n");
                let body = full_comment
                    .trim_start_matches("/*")
                    .trim_end_matches("*/");

                let mut line_count = 0;
                for comment_line in body.lines() {
                    let cl = comment_line.trim().trim_start_matches('*').trim();
                    if looks_like_code(cl) {
                        line_count += 1;
                    }
                }

                if line_count >= 2 && comment_lines.len() >= 3 {
                    findings.push(RuleFinding {
                        line: start_line + 1,
                        column: 1,
                        message: "Multi-line commented-out code detected".to_string(),
                    });
                }
            }
            i += 1;
        }

        findings
    }
}

fn looks_like_code(text: &str) -> bool {
    let t = text.trim();
    if t.is_empty() || t.len() < 3 {
        return false;
    }
    CODE_KEYWORDS.iter().any(|kw| t.contains(kw))
}
