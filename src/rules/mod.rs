use std::collections::HashMap;

use oxc_ast::ast::Program;
use oxc_semantic::Semantic;

pub mod nextjs;
pub mod performance;
pub mod quality;
pub mod react;
pub mod security;
pub mod testing;
pub mod typescript;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Off,
}

impl std::str::FromStr for Severity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(Severity::Error),
            "warn" | "warning" => Ok(Severity::Warning),
            _ => Ok(Severity::Off),
        }
    }
}

impl Severity {
    pub fn is_on(&self) -> bool {
        !matches!(self, Severity::Off)
    }
}

#[derive(Debug, Clone)]
pub struct RuleMeta {
    pub id: &'static str,
    pub default_severity: Severity,
    pub category: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct Violation {
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub rule_id: String,
    pub category: String,
    pub message: String,
    pub severity: Severity,
}

impl Violation {
    pub fn to_finding(&self) -> RuleFinding {
        RuleFinding {
            line: self.line,
            column: self.column,
            message: self.message.clone(),
        }
    }
}

pub trait Rule: Send + Sync {
    fn meta(&self) -> &RuleMeta;
    fn run(&self, program: &Program, semantic: &Semantic, source_text: &str) -> Vec<RuleFinding>;
    /// If this rule supports auto-fix, return the byte span and replacement text.
    /// Default implementation returns `None` (no fix available).
    fn fix(&self, _finding: &RuleFinding, _source_text: &str) -> Option<Fix> {
        None
    }

    /// Whether this rule has auto-fix capability.
    fn has_fix(&self) -> bool {
        false
    }
}

pub struct RuleFinding {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

pub struct Fix {
    pub start: usize,
    pub end: usize,
    pub replacement: String,
}

pub struct RuleRegistry {
    rules: Vec<Box<dyn Rule>>,
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleRegistry {
    pub fn new() -> Self {
        let mut registry = Self { rules: Vec::new() };
        registry.register_all();
        registry
    }

    fn register_all(&mut self) {
        // ── Phase 4: Code Quality ──
        self.rules.push(Box::new(quality::no_console::NoConsole));
        self.rules
            .push(Box::new(quality::no_empty_blocks::NoEmptyBlocks));
        self.rules.push(Box::new(quality::no_var::NoVar));
        self.rules.push(Box::new(quality::max_params::MaxParams));
        self.rules
            .push(Box::new(quality::no_long_functions::NoLongFunctions));
        self.rules
            .push(Box::new(quality::prefer_early_return::PreferEarlyReturn));
        self.rules
            .push(Box::new(quality::no_commented_code::NoCommentedCode));
        self.rules
            .push(Box::new(quality::no_deep_nesting::NoDeepNesting));
        self.rules
            .push(Box::new(quality::no_magic_numbers::NoMagicNumbers));
        self.rules
            .push(Box::new(quality::consistent_return::ConsistentReturn));
        self.rules
            .push(Box::new(quality::no_unused_vars_rule::NoUnusedVars));
        self.rules.push(Box::new(quality::no_shadow::NoShadow));
        self.rules.push(Box::new(quality::complexity::Complexity));
        self.rules
            .push(Box::new(quality::no_boolean_param::NoBooleanParam));
        self.rules
            .push(Box::new(quality::no_empty_catch::NoEmptyCatch));
        self.rules
            .push(Box::new(quality::no_global_mutation::NoGlobalMutation));
        self.rules.push(Box::new(
            quality::prefer_default_params::PreferDefaultParams,
        ));
        self.rules
            .push(Box::new(quality::prefer_object_spread::PreferObjectSpread));
        // ── Phase 5: React ──
        self.rules
            .push(Box::new(react::no_missing_key::NoMissingKey));
        self.rules
            .push(Box::new(react::no_inline_styles::NoInlineStyles));
        self.rules.push(Box::new(
            react::consistent_component_naming::ConsistentComponentNaming,
        ));
        self.rules.push(Box::new(react::no_index_key::NoIndexKey));
        self.rules
            .push(Box::new(react::no_inline_functions::NoInlineFunctions));
        self.rules.push(Box::new(
            react::prefer_function_components::PreferFunctionComponents,
        ));
        self.rules
            .push(Box::new(react::no_unnecessary_memo::NoUnnecessaryMemo));
        self.rules.push(Box::new(
            react::no_multiple_render_methods::NoMultipleRenderMethods,
        ));
        self.rules.push(Box::new(
            react::no_side_effects_in_render::NoSideEffectsInRender,
        ));
        self.rules.push(Box::new(react::hook_rules::HookRules));
        self.rules
            .push(Box::new(react::effect_deps_complete::EffectDepsComplete));
        self.rules
            .push(Box::new(react::no_set_state_in_effect::NoSetStateInEffect));
        self.rules
            .push(Box::new(react::no_set_state_in_render::NoSetStateInRender));
        self.rules
            .push(Box::new(react::no_duplicate_props::NoDuplicateProps));
        self.rules
            .push(Box::new(react::no_direct_mutation::NoDirectMutation));
        self.rules.push(Box::new(
            react::no_ref_in_component_name::NoRefInComponentName,
        ));
        self.rules
            .push(Box::new(react::no_forward_ref::NoForwardRef));
        self.rules
            .push(Box::new(react::no_array_index_key::NoArrayIndexKey));
        self.rules.push(Box::new(
            react::no_state_in_default_props::NoStateInDefaultProps,
        ));
        // ── Phase 6: TypeScript ──
        self.rules.push(Box::new(typescript::no_any::NoAny));
        self.rules.push(Box::new(
            typescript::no_non_null_assertion::NoNonNullAssertion,
        ));
        self.rules
            .push(Box::new(typescript::no_type_assertion::NoTypeAssertion));
        self.rules
            .push(Box::new(typescript::no_empty_interface::NoEmptyInterface));
        self.rules.push(Box::new(
            typescript::consistent_type_imports::ConsistentTypeImports,
        ));
        self.rules.push(Box::new(
            typescript::explicit_return_type::ExplicitReturnType,
        ));
        self.rules
            .push(Box::new(typescript::strict_null_checks::StrictNullChecks));
        self.rules
            .push(Box::new(typescript::prefer_interface::PreferInterface));
        self.rules
            .push(Box::new(typescript::no_explicit_any::NoExplicitAny));
        // ── Phase 7: Security ──
        self.rules.push(Box::new(
            security::no_dangerously_set_innerhtml::NoDangerouslySetInnerHtml,
        ));
        self.rules.push(Box::new(security::no_eval::NoEval));
        self.rules
            .push(Box::new(security::no_script_url::NoScriptUrl));
        self.rules
            .push(Box::new(security::no_hardcoded_secrets::NoHardcodedSecrets));
        self.rules
            .push(Box::new(security::no_unsanitized_input::NoUnsanitizedInput));
        self.rules
            .push(Box::new(security::no_insecure_protocol::NoInsecureProtocol));
        self.rules
            .push(Box::new(security::no_unsafe_iframe::NoUnsafeIframe));
        // ── Phase 8: Performance & Accessibility ──
        self.rules
            .push(Box::new(performance::prefer_fragments::PreferFragments));
        self.rules
            .push(Box::new(performance::no_bind_in_jsx::NoBindInJsx));
        self.rules.push(Box::new(performance::img_alt::ImgAlt));
        self.rules
            .push(Box::new(performance::button_has_type::ButtonHasType));
        self.rules
            .push(Box::new(performance::label_associated::LabelAssociated));
        self.rules.push(Box::new(
            performance::no_heavy_computation_in_render::NoHeavyComputationInRender,
        ));
        self.rules.push(Box::new(
            performance::lazy_load_components::LazyLoadComponents,
        ));
        self.rules
            .push(Box::new(performance::aria_valid::AriaValid));
        self.rules
            .push(Box::new(performance::heading_levels::HeadingLevels));
        self.rules
            .push(Box::new(performance::a_has_content::AHasContent));
        self.rules.push(Box::new(
            performance::no_ambiguous_labels::NoAmbiguousLabels,
        ));
        self.rules.push(Box::new(
            performance::tabindex_no_positive::TabindexNoPositive,
        ));
        self.rules.push(Box::new(
            performance::click_events_have_key_events::ClickEventsHaveKeyEvents,
        ));
        self.rules
            .push(Box::new(performance::html_has_lang::HtmlHasLang));
        self.rules
            .push(Box::new(performance::no_autofocus::NoAutofocus));
        // ── Phase 12: Next.js ──
        self.rules
            .push(Box::new(nextjs::no_img_element::NoImgElement));
        self.rules
            .push(Box::new(nextjs::no_script_tag_in_head::NoScriptTagInHead));
        self.rules.push(Box::new(nextjs::no_page_link::NoPageLink));
        self.rules
            .push(Box::new(nextjs::no_head_element::NoHeadElement));
        self.rules
            .push(Box::new(nextjs::no_sync_script::NoSyncScript));
        // ── Phase 14 continued: Performance ──
        self.rules
            .push(Box::new(performance::no_large_libraries::NoLargeLibraries));
        // ── Testing ──
        self.rules
            .push(Box::new(testing::no_skipped_tests::NoSkippedTests));
        self.rules.push(Box::new(
            testing::assert_includes_message::AssertIncludesMessage,
        ));
    }

    pub fn run_rules(
        &self,
        program: &Program,
        semantic: &Semantic,
        source_text: &str,
        file_path: &str,
        severity_overrides: &HashMap<String, String>,
        category_filter: Option<&Vec<String>>,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();

        for rule in &self.rules {
            let meta = rule.meta();

            if let Some(categories) = &category_filter
                && !categories.contains(&meta.category.to_string())
            {
                continue;
            }

            let effective_severity = severity_overrides
                .get(meta.id)
                .map(|s| s.parse::<Severity>().unwrap())
                .unwrap_or_else(|| meta.default_severity.clone());

            if !effective_severity.is_on() {
                continue;
            }

            let findings = rule.run(program, semantic, source_text);

            for finding in &findings {
                violations.push(Violation {
                    file_path: file_path.to_string(),
                    line: finding.line,
                    column: finding.column,
                    rule_id: meta.id.to_string(),
                    category: meta.category.to_string(),
                    message: finding.message.clone(),
                    severity: effective_severity.clone(),
                });
            }
        }

        violations
    }

    pub fn get_rule_ids(&self) -> Vec<&'static str> {
        self.rules.iter().map(|r| r.meta().id).collect()
    }

    pub fn get_rule(&self, rule_id: &str) -> Option<&dyn Rule> {
        self.rules
            .iter()
            .find(|r| r.meta().id == rule_id)
            .map(|v| v.as_ref())
    }

    /// Generate a default `.rauditrc.toml` config file with all rules listed.
    pub fn generate_config(&self) -> String {
        let mut buf = String::new();
        buf.push_str("# react-auditor default configuration\n");
        buf.push_str("# Adjust severity: \"error\", \"warning\", \"off\"\n");
        buf.push_str("\n# ── General ──\n");
        buf.push_str("# format = \"stylish\"\n");
        buf.push_str("# ignore = \"node_modules,dist,build\"\n");
        buf.push_str("# log = \"audit.json\"\n");
        buf.push_str("# max_warnings = 10\n\n");

        let mut metas: Vec<&RuleMeta> = self.rules.iter().map(|r| r.meta()).collect();
        metas.sort_by_key(|m| (m.category, m.id));

        let mut current_category = "";
        for meta in &metas {
            if meta.category != current_category {
                if !current_category.is_empty() {
                    buf.push('\n');
                }
                let title = format!("─ {} ─", capitalize(meta.category));
                let padded = format!(" {:─^70} ", title);
                buf.push_str(&format!("#{padded}\n"));
                current_category = meta.category;
            }
            let sev = match meta.default_severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Off => "off",
            };
            buf.push_str(&format!(
                "\"{}\" = \"{}\"   # {}\n",
                meta.id, sev, meta.description
            ));
        }

        buf
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn line_col_to_offset(source: &str, line: usize, col: usize) -> Option<usize> {
    let mut current_line = 1;
    let mut offset = 0;
    for (i, _) in source.char_indices() {
        if current_line == line {
            return Some(offset + col - 1);
        }
        if source.as_bytes().get(i) == Some(&b'\n') {
            current_line += 1;
            offset = i + 1;
        }
    }
    None
}
