// orqa-plugin: Plugin system for the OrqaStudio engine.
//
// This crate provides the complete plugin lifecycle: reading manifests,
// discovering installed plugins, detecting key collisions, installing from
// local paths or GitHub releases, managing the lockfile, and browsing the
// plugin registry.
//
// The engine plugin system implements P1 (Plugin-Composed Everything): all
// governance definitions come from plugins; the engine provides the capability
// infrastructure to manage them.

pub mod cli_runner;
pub mod collision;
pub mod constraints;
pub mod discovery;
pub mod hooks;
pub mod installer;
pub mod lockfile;
pub mod manifest;
pub mod registry;
