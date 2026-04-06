//! Build script for orqa-validation.
//!
//! Reads all `*.schema.json` files from `../types/src/platform/` and generates
//! Rust types. The generated code is written to `src/generated/` and checked in
//! so the crate can be built without the build script running (e.g. in downstream
//! consumers that vendor the crate).
//!
//! To regenerate after schema changes:
//!
//! ```
//! cargo build -p orqa-validation
//! ```
//!
//! The generated files will be updated in-place under `src/generated/`.

// Build scripts are not library code. The restrictions below are suppressed:
// - missing_docs: build scripts are not published APIs
// - unwrap_used: .unwrap() in build scripts panics with a clear build error message
// - print_stderr: eprintln! is Cargo's API for build warnings (cargo:warning=...)
// - too_many_lines: code-generation functions are intentionally long
#![allow(
    clippy::too_many_lines,
    clippy::unwrap_used,
    clippy::print_stderr,
    missing_docs
)]

use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

fn main() {
    // Tell Cargo to re-run this script if any schema file changes.
    let schema_dir = Path::new("../types/src/platform");
    if schema_dir.exists() {
        println!("cargo:rerun-if-changed=../types/src/platform");
        if let Ok(entries) = fs::read_dir(schema_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) == Some("json")
                    && p.file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.ends_with(".schema.json"))
                {
                    println!("cargo:rerun-if-changed={}", p.display());
                }
            }
        }
    }

    // Generate Rust types from each schema file.
    let output_dir = Path::new("src/generated");
    if let Err(e) = generate_rust_types(schema_dir, output_dir) {
        // Non-fatal: if the schema directory doesn't exist (e.g. vendored build),
        // skip generation and use the checked-in files.
        eprintln!("cargo:warning=Schema generation skipped: {e}");
    }
}

fn generate_rust_types(
    schema_dir: &Path,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if !schema_dir.exists() {
        return Err(format!("Schema directory not found: {}", schema_dir.display()).into());
    }

    fs::create_dir_all(output_dir)?;

    let mut schema_files: Vec<_> = fs::read_dir(schema_dir)?
        .flatten()
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".schema.json"))
        })
        .collect();

    // Sort for deterministic output.
    schema_files.sort_by_key(fs::DirEntry::path);

    if schema_files.is_empty() {
        return Ok(());
    }

    let mut module_names: Vec<String> = Vec::new();

    for entry in &schema_files {
        let schema_path = entry.path();
        let file_stem = schema_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("invalid filename")?;
        // Remove the ".schema" suffix from the stem (file_stem gives us "graph.schema").
        let module_name = file_stem
            .strip_suffix(".schema")
            .unwrap_or(file_stem)
            .replace('-', "_");

        let schema_content = fs::read_to_string(&schema_path)?;
        let schema_json: serde_json::Value = serde_json::from_str(&schema_content)?;

        let rust_code = schema_to_rust(&schema_json)?;

        let output_file = output_dir.join(format!("{module_name}.rs"));
        fs::write(&output_file, rust_code)?;
        module_names.push(module_name);
    }

    // Write the mod.rs that declares and re-exports all generated modules.
    let mod_content = generate_mod_rs(&module_names);
    fs::write(output_dir.join("mod.rs"), mod_content)?;

    Ok(())
}

/// Rust reserved keywords that need `r#` escaping when used as field names.
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

/// Escape a field name if it is a Rust reserved keyword.
fn escape_keyword(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("r#{name}")
    } else {
        name.to_owned()
    }
}

/// Convert a JSON Schema `$defs` block to Rust struct/enum definitions.
///
/// Handles:
/// - Object types → `pub struct`
/// - String enums → `pub enum`
/// - Optional fields → `Option<T>`
/// - Nullable types (`["string", "null"]`) → `Option<T>`
/// - Arrays → `Vec<T>`
/// - Integer → `usize` (minimum >= 0) or `i64`
/// - Number → `f64`
/// - `$ref` → referenced type name
fn schema_to_rust(schema: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let defs = schema
        .get("$defs")
        .and_then(|v| v.as_object())
        .ok_or("schema has no $defs")?;

    let mut output = String::new();
    output.push_str("// THIS FILE IS AUTO-GENERATED — DO NOT EDIT BY HAND.\n");
    output.push_str("// Source: libs/types/src/platform/*.schema.json\n");
    output.push_str("// Regenerate: cargo build -p orqa-validation\n");
    output.push('\n');
    output.push_str("#![allow(dead_code, unused_imports, missing_docs)]\n");
    output.push('\n');
    output.push_str("use serde::{Deserialize, Serialize};\n");
    output.push('\n');

    // Build a set of all defined type names for cross-reference resolution.
    let defined_types: std::collections::HashSet<String> = defs.keys().cloned().collect();

    // Topological sort: emit types that have no unresolved forward references first.
    // Simple approach: emit in $defs declaration order; the generator uses fully-qualified
    // `Box<T>` for recursive types when needed.
    for (type_name, type_def) in defs {
        if let Some(s) = classify_and_generate(type_name, type_def, &defined_types) {
            output.push_str(&s);
            output.push('\n');
        }
    }

    Ok(output)
}

fn classify_and_generate(
    name: &str,
    def: &serde_json::Value,
    defined_types: &std::collections::HashSet<String>,
) -> Option<String> {
    // String enum.
    if def.get("type").and_then(|v| v.as_str()) == Some("string") && def.get("enum").is_some() {
        return Some(generate_enum(name, def));
    }

    // Object type — regular struct.
    if def.get("type").and_then(|v| v.as_str()) == Some("object") {
        return Some(generate_struct(name, def, defined_types));
    }

    // oneOf with multiple non-null variants → untagged enum.
    // Counts both `type: "object"` variants and `$ref` variants.
    if let Some(one_of) = def.get("oneOf").and_then(|v| v.as_array()) {
        let non_null_count = one_of
            .iter()
            .filter(|v| v.get("type").and_then(|t| t.as_str()) != Some("null"))
            .count();
        if non_null_count > 1 {
            return Some(generate_untagged_enum(name, def, one_of, defined_types));
        }
    }

    // No type field but has properties → treat as object.
    if def.get("properties").is_some() {
        return Some(generate_struct(name, def, defined_types));
    }

    None
}

#[allow(clippy::too_many_lines)]
fn generate_struct(
    name: &str,
    def: &serde_json::Value,
    defined_types: &std::collections::HashSet<String>,
) -> String {
    let description = def
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let properties = def
        .get("properties")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();

    let required: Vec<String> = def
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();

    let mut out = String::new();
    if !description.is_empty() {
        for line in description.lines() {
            writeln!(out, "/// {line}").unwrap();
        }
    }
    out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    writeln!(out, "pub struct {name} {{").unwrap();

    for (field_name, field_def) in &properties {
        // Skip fields with names that cannot be represented as Rust identifiers.
        // The `$schema` field (used in JSON Schema docs) is not meaningful at runtime.
        if field_name.starts_with('$') {
            continue;
        }

        let is_required = required.contains(field_name);
        let field_doc = field_def
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if !field_doc.is_empty() {
            // Doc comment — one line only to keep generated code clean.
            let first_line = field_doc.lines().next().unwrap_or(field_doc);
            writeln!(out, "    /// {first_line}").unwrap();
        }

        let rust_type = json_type_to_rust(field_def, defined_types);

        // Determine if this field needs to be wrapped in Option.
        // Fields not in `required` are optional. If the type is already Option<T>
        // (from a nullable schema), keep it as-is.
        let final_type = if is_required || rust_type.starts_with("Option<") {
            rust_type.clone()
        } else {
            format!("Option<{rust_type}>")
        };

        // Serde attributes.
        let mut serde_attrs: Vec<String> = Vec::new();

        // Rename if the JSON field name != the snake_case Rust name.
        let snake_name = to_snake_case(field_name);
        if snake_name != *field_name {
            serde_attrs.push(format!("rename = \"{field_name}\""));
        }

        // Skip None for optional fields.
        if !is_required {
            serde_attrs.push("skip_serializing_if = \"Option::is_none\"".to_owned());
            serde_attrs.push("default".to_owned());
        }

        if !serde_attrs.is_empty() {
            let attrs_str = serde_attrs.join(", ");
            writeln!(out, "    #[serde({attrs_str})]").unwrap();
        }

        let safe_name = escape_keyword(&snake_name);
        writeln!(out, "    pub {safe_name}: {final_type},").unwrap();
    }

    out.push_str("}\n");
    out
}

fn generate_enum(name: &str, def: &serde_json::Value) -> String {
    let description = def
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let variants: Vec<String> = def
        .get("enum")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();

    let mut out = String::new();
    if !description.is_empty() {
        let first_line = description.lines().next().unwrap_or(description);
        writeln!(out, "/// {first_line}").unwrap();
    }
    out.push_str("#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]\n");
    writeln!(out, "pub enum {name} {{").unwrap();

    for variant in &variants {
        // Preserve PascalCase enum values — do not lowercase.
        let rust_variant = to_pascal_case(variant);
        if rust_variant != *variant {
            writeln!(out, "    #[serde(rename = \"{variant}\")]").unwrap();
        }
        writeln!(out, "    {rust_variant},").unwrap();
    }

    out.push_str("}\n");
    out
}

fn generate_untagged_enum(
    name: &str,
    def: &serde_json::Value,
    variants: &[serde_json::Value],
    defined_types: &std::collections::HashSet<String>,
) -> String {
    let description = def
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let mut out = String::new();
    if !description.is_empty() {
        let first_line = description.lines().next().unwrap_or(description);
        writeln!(out, "/// {first_line}").unwrap();
    }
    out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    out.push_str("#[serde(untagged)]\n");
    writeln!(out, "pub enum {name} {{").unwrap();

    for (i, variant) in variants.iter().enumerate() {
        // Generate inline struct variant or $ref variant.
        if let Some(ref_val) = variant.get("$ref").and_then(|v| v.as_str()) {
            if let Some(def_name) = ref_val.strip_prefix("#/$defs/") {
                if defined_types.contains(def_name) {
                    writeln!(out, "    Variant{i}({def_name}),").unwrap();
                    continue;
                }
            }
        }
        // Null variant → unit.
        if variant.get("type").and_then(|v| v.as_str()) == Some("null") {
            writeln!(out, "    Variant{i}(()),").unwrap();
            continue;
        }
        // Other variants — use serde_json::Value as a catch-all.
        writeln!(out, "    Variant{i}(serde_json::Value),").unwrap();
    }

    out.push_str("}\n");
    out
}

#[allow(clippy::too_many_lines)]
fn json_type_to_rust(
    field_def: &serde_json::Value,
    defined_types: &std::collections::HashSet<String>,
) -> String {
    // Handle anyOf with null (nullable types produced by `["string", "null"]`).
    if let Some(type_arr) = field_def.get("type").and_then(|v| v.as_array()) {
        let types: Vec<&str> = type_arr.iter().filter_map(|v| v.as_str()).collect();
        if types.contains(&"null") {
            let non_null: Vec<&str> = types.iter().copied().filter(|&t| t != "null").collect();
            if non_null.len() == 1 {
                let inner = primitive_type_to_rust(non_null[0], field_def);
                return format!("Option<{inner}>");
            }
        }
        if types.len() == 1 {
            return primitive_type_to_rust(types[0], field_def);
        }
    }

    // Handle $ref.
    if let Some(ref_val) = field_def.get("$ref").and_then(|v| v.as_str()) {
        if let Some(def_name) = ref_val.strip_prefix("#/$defs/") {
            if defined_types.contains(def_name) {
                return def_name.to_owned();
            }
        }
        return "serde_json::Value".to_owned();
    }

    // Handle oneOf with null (e.g. parent: DeliveryParentConfig | null).
    if let Some(one_of) = field_def.get("oneOf").and_then(|v| v.as_array()) {
        let non_null: Vec<&serde_json::Value> = one_of
            .iter()
            .filter(|v| v.get("type").and_then(|t| t.as_str()) != Some("null"))
            .collect();
        let has_null = one_of
            .iter()
            .any(|v| v.get("type").and_then(|t| t.as_str()) == Some("null"));

        if non_null.len() == 1 {
            let inner = json_type_to_rust(non_null[0], defined_types);
            if has_null {
                return format!("Option<{inner}>");
            }
            return inner;
        }
    }

    // Handle primitive type string.
    if let Some(type_str) = field_def.get("type").and_then(|v| v.as_str()) {
        return match type_str {
            "array" => {
                if let Some(items) = field_def.get("items") {
                    let item_type = json_type_to_rust(items, defined_types);
                    format!("Vec<{item_type}>")
                } else {
                    "Vec<serde_json::Value>".to_owned()
                }
            }
            "object" => {
                if let Some(add_props) = field_def.get("additionalProperties") {
                    if add_props.as_bool() == Some(true) || add_props.as_bool() == Some(false) {
                        return "serde_json::Value".to_owned();
                    }
                    let val_type = json_type_to_rust(add_props, defined_types);
                    return format!("std::collections::HashMap<String, {val_type}>");
                }
                "serde_json::Value".to_owned()
            }
            other => primitive_type_to_rust(other, field_def),
        };
    }

    "serde_json::Value".to_owned()
}

fn primitive_type_to_rust(type_str: &str, field_def: &serde_json::Value) -> String {
    match type_str {
        "string" => "String".to_owned(),
        "boolean" => "bool".to_owned(),
        "integer" => {
            // Use usize if minimum >= 0, otherwise i64.
            let min = field_def
                .get("minimum")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(-1.0);
            if min >= 0.0 {
                "usize".to_owned()
            } else {
                "i64".to_owned()
            }
        }
        "number" => "f64".to_owned(),
        _ => "serde_json::Value".to_owned(),
    }
}

fn generate_mod_rs(module_names: &[String]) -> String {
    let mut out = String::new();
    out.push_str("// THIS FILE IS AUTO-GENERATED — DO NOT EDIT BY HAND.\n");
    out.push_str("// Source: libs/types/src/platform/*.schema.json\n");
    out.push_str("// Regenerate: cargo build -p orqa-validation\n");
    out.push('\n');
    out.push_str("#![allow(dead_code, missing_docs)]\n");
    out.push('\n');
    for name in module_names {
        writeln!(out, "pub mod {name};").unwrap();
    }
    out.push('\n');
    // Re-export all types from all modules.
    for name in module_names {
        writeln!(out, "pub use {name}::*;").unwrap();
    }
    out
}

/// Convert a camelCase or kebab-case string to snake_case.
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_upper = false;
    for (i, ch) in s.chars().enumerate() {
        if ch == '-' || ch == '_' {
            result.push('_');
            prev_upper = false;
        } else if ch.is_ascii_uppercase() {
            if i > 0 && !prev_upper {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
            prev_upper = true;
        } else {
            result.push(ch);
            prev_upper = false;
        }
    }
    result
}

/// Convert a kebab-case, snake_case, or already-PascalCase string to PascalCase.
///
/// If the string contains no separators (hyphens or underscores), it is assumed
/// to already be PascalCase and is returned unchanged.
fn to_pascal_case(s: &str) -> String {
    if !s.contains('-') && !s.contains('_') {
        // Already PascalCase (or a single word) — preserve as-is.
        return s.to_owned();
    }
    s.split(['-', '_'])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut result = first.to_uppercase().to_string();
                    result.push_str(&chars.as_str().to_lowercase());
                    result
                }
            }
        })
        .collect()
}

// Note: the include_str! that previously verified graph.schema.json at compile time
// has been removed. Plugins are now the source of truth for schemas; the
// libs/types/src/platform/ directory no longer exists.
