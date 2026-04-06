// Route modules for the daemon HTTP API.
//
// Each module corresponds to a resource group in the API design:
//   - artifacts:   CRUD and graph queries for individual artifacts
//   - graph:       graph-level analytics (stats, health, snapshots)
//   - validation:  integrity scan, auto-fix, hook evaluation
//   - enforcement: rule listing, reload, violation scan
//   - search:      index, embed, regex, semantic search
//   - workflow:    status transition evaluation and application
//   - prompt:      system prompt generation and knowledge injection
//   - plugins:     plugin lifecycle (list, install, uninstall, registry)
//   - agents:      agent preamble and behavioral message extraction
//   - content:     knowledge artifact loading
//   - lessons:     lesson CRUD and recurrence tracking
//   - sessions:    session and message management (SQLite)
//   - streaming:   daemon-side stream loop with SSE delivery (POST/GET/stop/tool-approval)
//   - projects:    project management, settings, scan, icon
//   - settings:    app settings key/value store (SQLite)
//   - sidecar:     managed subprocess status and restart
//   - cli_tools:   plugin-registered CLI tool execution
//   - hooks:       plugin hook registry and dispatcher generation
//   - setup:       setup wizard status and prerequisite checks
//   - devtools:    OrqaDev devtools window launch and status
//   - git:         git stash list and uncommitted status
//   - startup:     daemon startup task status

pub mod agents;
pub mod artifacts;
pub mod cli_tools;
pub mod content;
pub mod devtools;
pub mod enforcement;
pub mod git;
pub mod graph;
pub mod hooks;
pub mod lessons;
pub mod plugins;
pub mod projects;
pub mod prompt;
pub mod search;
pub mod sessions;
pub mod settings;
pub mod setup;
pub mod sidecar;
pub mod startup;
pub mod streaming;
pub mod validation;
pub mod workflow;
