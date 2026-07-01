use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Rule, RuleFinding, RuleMeta, Severity};

pub struct NoSyncScript;

const RULE_META: RuleMeta = RuleMeta {
    id: "no-sync-script",
    default_severity: Severity::Warning,
    category: "nextjs",
    description: "Use `strategy=\"afterInteractive\"` on `<Script>` from `next/script`",
};

impl Rule for NoSyncScript {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = SyncScriptCollector {
            findings: Vec::new(),
            source: source_text,
            script_imported: false,
        };
        collector.visit_program(program);
        collector.findings
    }
}

struct SyncScriptCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
    script_imported: bool,
}

impl<'a> SyncScriptCollector<'a> {
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

impl<'a> Visit<'a> for SyncScriptCollector<'a> {
    fn visit_import_declaration(&mut self, decl: &oxc_ast::ast::ImportDeclaration<'a>) {
        if decl.source.value.as_str() == "next/script" {
            self.script_imported = true;
        }
    }

    fn visit_jsx_opening_element(&mut self, el: &oxc_ast::ast::JSXOpeningElement<'a>) {
        if !self.script_imported {
            return;
        }

        let is_script = matches!(
            &el.name,
            oxc_ast::ast::JSXElementName::IdentifierReference(id)
                if id.name.as_str() == "Script"
        );

        if !is_script {
            return;
        }

        let mut has_strategy = false;
        for attr_item in &el.attributes {
            if let oxc_ast::ast::JSXAttributeItem::Attribute(attr) = attr_item
                && let oxc_ast::ast::JSXAttributeName::Identifier(id) = &attr.name
                && id.name.as_str() == "strategy"
            {
                has_strategy = true;
                if let Some(val) = &attr.value
                    && let oxc_ast::ast::JSXAttributeValue::StringLiteral(s) = val
                    && s.value.as_str() == "afterInteractive"
                {
                    return;
                }
            }
        }

        if has_strategy {
            self.add_finding(
                el.span.start as usize,
                "`<Script>` with `strategy` should use `\"afterInteractive\"` for better performance"
                    .to_string(),
            );
        } else {
            self.add_finding(
                el.span.start as usize,
                "`<Script>` from `next/script` should have `strategy=\"afterInteractive\"`"
                    .to_string(),
            );
        }
    }
}
