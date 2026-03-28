fn main() {
    // DuckDB on Windows needs rstrtmgr.lib for Restart Manager API
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=rstrtmgr");
}
