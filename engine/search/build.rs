//! Build script for orqa-search — links platform-specific libraries required by DuckDB.
// Build scripts are not library code; doc and lint requirements do not apply.
#![allow(missing_docs, clippy::unwrap_used)]

fn main() {
    // DuckDB on Windows needs rstrtmgr.lib for Restart Manager API
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=rstrtmgr");
}
