#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(source) = std::str::from_utf8(data) {
        let allocator = oxc_allocator::Allocator::default();
        let parser = oxc_parser::Parser::new(&allocator, source, oxc_parser::ParserOptions::default());
        if let Ok(ret) = parser.parse() {
            let program = ret.program;
            let semantic = oxc_semantic::SemanticBuilder::new()
                .build(&program)
                .semantic;
            let registry = react_auditor::rules::RuleRegistry::new();
            registry.run_rules(&program, &semantic, source, "fuzz.jsx", &std::collections::HashMap::new(), None);
        }
    }
});
