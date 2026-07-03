use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;
use oxc_semantic::Semantic;

use crate::rules::{Fix, Rule, RuleFinding, RuleMeta, Severity};

pub struct PreferFunctionComponents;

const RULE_META: RuleMeta = RuleMeta {
    id: "prefer-function-components",
    default_severity: Severity::Warning,
    category: "react",
    description: "Prefer function components over class components",
};

impl Rule for PreferFunctionComponents {
    fn meta(&self) -> &RuleMeta {
        &RULE_META
    }

    fn run(&self, program: &Program, _semantic: &Semantic, source_text: &str) -> Vec<RuleFinding> {
        let mut collector = ClassComponentCollector {
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
        let class_name_offset =
            crate::rules::line_col_to_offset(source_text, finding.line, finding.column)?;

        let before = &source_text[..class_name_offset];
        let class_keyword_start = before.rfind("class")?;
        let after_name = &source_text[class_name_offset..];

        let body_open = after_name.find('{')?;
        let body_start = class_name_offset + body_open + 1;

        let body_end = find_matching_brace(source_text, body_start - 1)?;

        let class_body = &source_text[body_start..body_end];

        let render_body = extract_render_body(source_text, body_start, body_end)?;

        let state_init = extract_state_initializer(class_body);

        let uses_props = class_body.contains("this.props");
        let has_lifecycle = has_lifecycle_method(class_body);

        if has_lifecycle {
            return None;
        }

        let props_param = if uses_props { "props" } else { "_props" };

        let mut fn_body = String::new();

        if let Some((_state_vars, state_values)) = state_init {
            let state_var_count = count_state_vars(&state_values);
            if state_var_count == 0 {
                fn_body.push_str("  // TODO: migrate state manually\n");
            } else {
                let var_names: Vec<String> = (0..state_var_count)
                    .map(|i| {
                        format!(
                            "state{}",
                            if i == 0 { String::new() } else { i.to_string() }
                        )
                    })
                    .collect();
                let setter_names: Vec<String> = (0..state_var_count)
                    .map(|i| {
                        format!(
                            "setState{}",
                            if i == 0 { String::new() } else { i.to_string() }
                        )
                    })
                    .collect();
                fn_body.push_str(&format!(
                    "  const [{}] = useState({});\n\n",
                    var_names
                        .iter()
                        .zip(setter_names.iter())
                        .map(|(v, s)| format!("{v}, {s}"))
                        .collect::<Vec<_>>()
                        .join(", "),
                    state_values,
                ));
            }
        }

        fn_body.push_str(&render_body);

        let class_name = find_class_name(after_name)?;

        let result = format!("function {class_name}({props_param}) {{\n{fn_body}}}");

        Some(Fix {
            start: class_keyword_start,
            end: body_end + 1,
            replacement: result,
        })
    }
}

fn find_matching_brace(source: &str, open_pos: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    if open_pos >= bytes.len() || bytes[open_pos] != b'{' {
        return None;
    }
    let mut depth = 1u32;
    let mut i = open_pos + 1;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            depth += 1;
        } else if bytes[i] == b'}' {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn extract_render_body(source: &str, body_start: usize, body_end: usize) -> Option<String> {
    let body = &source[body_start..body_end];
    let render_pos = body.find("render")?;
    let after_render = &body[render_pos..];
    let paren_start = after_render.find('(')?;
    let paren_end = after_render[paren_start..].find(')')? + paren_start;
    let after_paren = &after_render[paren_end..];
    let brace_start = after_paren.find('{')?;
    let render_brace_start = body_start + render_pos + paren_end + brace_start;
    let render_brace_end = find_matching_brace(source, render_brace_start + 1)?;

    let render_content = &source[render_brace_start + 1..render_brace_end];
    let trimmed = render_content.trim();

    let mut body = String::with_capacity(trimmed.len() + 32);
    for line in trimmed.lines() {
        body.push_str("  ");
        body.push_str(line.trim());
        body.push('\n');
    }

    Some(body)
}

fn extract_state_initializer(class_body: &str) -> Option<(Vec<String>, String)> {
    let constructor_pos = class_body.find("constructor")?;
    let after_ctor = &class_body[constructor_pos..];
    let brace_start = after_ctor.find('{')?;
    let after_brace = &after_ctor[brace_start + 1..];
    let this_state_pos = after_brace.find("this.state = ")?;
    let assign_start = this_state_pos + "this.state = ".len();
    let remaining = &after_brace[assign_start..];
    let obj_brace = remaining.find('{')?;
    let obj_end = find_matching_brace_inner(remaining, obj_brace)?;

    let state_obj = &remaining[..=obj_end];
    Some((Vec::new(), state_obj.to_string()))
}

fn find_matching_brace_inner(s: &str, open_pos: usize) -> Option<usize> {
    let bytes = s.as_bytes();
    if open_pos >= bytes.len() || bytes[open_pos] != b'{' {
        return None;
    }
    let mut depth = 1u32;
    let mut i = open_pos + 1;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            depth += 1;
        } else if bytes[i] == b'}' {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn count_state_vars(state_values: &str) -> usize {
    let mut count = 0;
    let mut in_string = false;
    let mut in_template = false;
    let bytes = state_values.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' | b'\'' if !in_template => in_string = !in_string,
            b'`' if !in_string => in_template = !in_template,
            b':' if !in_string && !in_template => count += 1,
            _ => {}
        }
        i += 1;
    }
    count
}

fn find_class_name(after_name: &str) -> Option<String> {
    let trimmed = after_name.trim_start();
    let end = trimmed.find(|c: char| c.is_whitespace() || c == '{' || c == '<')?;
    Some(trimmed[..end].to_string())
}

fn has_lifecycle_method(class_body: &str) -> bool {
    let lifecycle_methods = [
        "componentDidMount",
        "componentDidUpdate",
        "componentWillUnmount",
        "shouldComponentUpdate",
        "getDerivedStateFromProps",
        "getSnapshotBeforeUpdate",
        "componentDidCatch",
        "UNSAFE_componentWillMount",
        "UNSAFE_componentWillReceiveProps",
        "UNSAFE_componentWillUpdate",
    ];
    lifecycle_methods.iter().any(|m| class_body.contains(m))
}

struct ClassComponentCollector<'a> {
    findings: Vec<RuleFinding>,
    source: &'a str,
}

impl<'a> Visit<'a> for ClassComponentCollector<'a> {
    fn visit_class(&mut self, class: &oxc_ast::ast::Class<'a>) {
        let extends_react = class.super_class.as_ref().is_some_and(|expr| {
            if let oxc_ast::ast::Expression::Identifier(ident) = expr {
                let name = ident.name.as_str();
                name == "Component" || name == "PureComponent"
            } else if let Some(member) = expr.as_member_expression() {
                member
                    .static_property_name()
                    .is_some_and(|n| n == "Component" || n == "PureComponent")
            } else {
                false
            }
        });

        if extends_react && let Some(id) = &class.id {
            let start = id.span.start as usize;
            let line = self.source[..start].lines().count().max(1);
            let col = start - self.source[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.findings.push(RuleFinding {
                line,
                column: col + 1,
                message: format!(
                    "`{}` extends Component — prefer a function component",
                    id.name
                ),
            });
        }
    }
}
