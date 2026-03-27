// Enforcement rule parser — re-exported from the orqa-engine crate.
//
// `parse_rule_content` parses YAML frontmatter from enforcement rule `.md` files
// into typed `EnforcementRule` values. The implementation lives in
// orqa_engine::enforcement::parser and is consumed here without duplication.

pub use orqa_engine::enforcement::parser::parse_rule_content;
