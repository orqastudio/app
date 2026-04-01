//! OrqaDev application binary entry point.
//!
//! Launches the Tauri GUI for developer tools.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    orqa_devtools_lib::run();
}
