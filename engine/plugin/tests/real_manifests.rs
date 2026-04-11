//! Integration test: every real plugin manifest in `plugins/**/orqa-plugin.json`
//! must deserialize cleanly against [`orqa_plugin::manifest::PluginManifest`].
//!
//! This guards against schema drift between the Rust struct and the JSON
//! files first-party plugins ship.  Before this test existed, the daemon
//! silently skipped seven plugins at startup because the `AgentDefinition`
//! struct required `title` and `ActionDeclaration` required `files` — both
//! of which are absent from the shipped manifests.  Running this test on
//! CI ensures every first-party manifest round-trips through the loader.

use std::fs;
use std::path::{Path, PathBuf};

use orqa_plugin::manifest::PluginManifest;

/// Walk `dir` recursively and return every path whose file name is
/// `orqa-plugin.json`.
fn find_manifests(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(read) = fs::read_dir(dir) else {
        return;
    };
    for entry in read.flatten() {
        let path = entry.path();
        if path.is_dir() {
            find_manifests(&path, out);
        } else if path.file_name().and_then(|s| s.to_str()) == Some("orqa-plugin.json") {
            out.push(path);
        }
    }
}

/// Locate the repository root by walking upward from `CARGO_MANIFEST_DIR`
/// until we find a `plugins/` directory.
fn repo_root() -> PathBuf {
    let mut dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    while !dir.join("plugins").is_dir() {
        assert!(
            dir.pop(),
            "could not locate repo root from CARGO_MANIFEST_DIR"
        );
    }
    dir
}

#[test]
fn every_first_party_manifest_deserializes() {
    let root = repo_root();

    // Scan every directory that ships first-party plugins.  Keep this list
    // in sync with `project.json`'s `plugins` entries: if we register a new
    // plugin root there, add it here too so the regression net stays tight.
    let scan_dirs = [
        root.join("plugins"),
        root.join("connectors"),
        root.join("sidecars"),
    ];

    let mut manifests = Vec::new();
    for dir in &scan_dirs {
        if dir.is_dir() {
            find_manifests(dir, &mut manifests);
        }
    }

    assert!(
        !manifests.is_empty(),
        "expected at least one orqa-plugin.json under plugins/, connectors/, or sidecars/"
    );

    let mut failures = Vec::new();
    for path in &manifests {
        let raw = fs::read_to_string(path).expect("read manifest");
        match serde_json::from_str::<PluginManifest>(&raw) {
            Ok(_) => {}
            Err(e) => failures.push(format!("{}: {e}", path.display())),
        }
    }

    assert!(
        failures.is_empty(),
        "{} plugin manifest(s) failed to deserialize:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
