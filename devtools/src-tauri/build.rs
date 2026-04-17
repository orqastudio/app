//! Tauri build script — creates a minimal frontend placeholder when the build
//! directory is missing, then invokes tauri_build.
//!
//! During development, `devtools/build/` is produced by `npm run build`. On a
//! fresh checkout that directory does not exist, which causes tauri_build and
//! the tauri::generate_context!() proc-macro to panic. This script writes a
//! minimal placeholder so `cargo check` works without requiring a frontend
//! build first. The placeholder is overwritten at release time by `npm run build`.
// Build scripts are not library code; doc and lint requirements do not apply.
#![allow(missing_docs, clippy::unwrap_used)]

use std::fs;
use std::path::Path;

fn main() {
    // Resolve the frontend dist directory relative to this manifest's location.
    // CARGO_MANIFEST_DIR is set by Cargo and points to devtools/src-tauri/.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_dir = Path::new(&manifest_dir).join("../build");

    if !build_dir.exists() {
        fs::create_dir_all(&build_dir).unwrap();
        fs::write(
            build_dir.join("index.html"),
            "<!-- dev placeholder: run npm run build in devtools/ to replace -->\n",
        )
        .unwrap();
    }

    tauri_build::build();
}
