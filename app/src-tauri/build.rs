//! Tauri build script — invokes tauri_build to generate platform-specific resources.
// Build scripts are not library code; doc and lint requirements do not apply.
#![allow(missing_docs, clippy::unwrap_used)]

fn main() {
    tauri_build::build();
}
