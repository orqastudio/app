//! orqa-plugin: Plugin system for the OrqaStudio engine.
//!
//! This crate provides the complete plugin lifecycle: reading manifests,
//! discovering installed plugins, detecting key collisions, installing from
//! local paths or GitHub releases, managing the lockfile, and browsing the
//! plugin registry.
//!
//! The engine plugin system implements P1 (Plugin-Composed Everything): all
//! governance definitions come from plugins; the engine provides the capability
//! infrastructure to manage them.

/// One-shot CLI tool runner for plugin-registered CLI tools.
pub mod cli_runner;
/// Key collision detection between plugins and core.
pub mod collision;
/// Installation constraint enforcement (one-methodology, one-per-stage).
pub mod constraints;
/// Content installation — runtime file copy and uninstall for plugin content entries.
pub mod content;
/// Plugin discovery — scan for installed and enabled plugins.
pub mod discovery;
/// Hook execution — run registered plugin hooks at lifecycle events.
pub mod hooks;
/// Plugin installer — install plugins from local paths or GitHub releases.
pub mod installer;
/// Plugin lockfile — read/write `plugins.lock.json`.
pub mod lockfile;
/// Plugin manifest reader — Rust representation of `orqa-plugin.json`.
pub mod manifest;
/// Plugin registry — fetch and cache official and community plugin catalogs.
pub mod registry;
