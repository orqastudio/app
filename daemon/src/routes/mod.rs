// Route modules for the daemon HTTP API.
//
// Each module corresponds to a resource group in the API design:
//   - admin_migrate:      storage migration routes (ingest phase)
//   - artifacts:          CRUD and graph queries for individual artifacts
//   - import:             POST /artifacts/import — markdown tree → SurrealDB with conflict policy
//   - graph:              graph-level analytics (stats, health, snapshots)
//   - validation:         integrity scan, auto-fix, hook evaluation
//   - enforcement:        rule listing, reload, violation scan
//   - search:             index, embed, regex, semantic search
//   - workflow:           status transition evaluation and application
//   - prompt:             system prompt generation and knowledge injection
//   - plugins:            plugin lifecycle (list, install, uninstall, registry)
//   - agents:             agent preamble and behavioral message extraction
//   - content:            knowledge artifact loading
//   - lessons:            lesson CRUD and recurrence tracking
//   - sessions:           session and message management (SQLite)
//   - messages:           message create, tool-message create, FTS5 search
//   - streaming:          daemon-side stream loop with SSE delivery (POST/GET/stop/tool-approval)
//   - projects:           project management, settings, scan, icon
//   - settings:           app settings key/value store (SQLite)
//   - themes:             project theme tokens and user overrides (SQLite)
//   - health_snapshots:   point-in-time artifact graph health metrics (SQLite)
//   - violations:         recorded enforcement violations (SQLite)
//   - devtools_sessions:  devtools session lifecycle and event query (SQLite)
//   - issue_groups:       deduplicated error clusters with sparkline (SQLite)
//   - sidecar:            managed subprocess status and restart
//   - cli_tools:          plugin-registered CLI tool execution
//   - hooks:              plugin hook registry and dispatcher generation
//   - setup:              setup wizard status and prerequisite checks
//   - devtools:           OrqaDev devtools window launch and status
//   - git:                git stash list and uncommitted status
//   - startup:            daemon startup task status
//   - watcher:            watcher pause/resume control

pub mod admin_migrate;
pub mod agents;
pub mod artifacts;
pub mod cli_tools;
pub mod content;
pub mod devtools;
pub mod devtools_sessions;
pub mod enforcement;
pub mod git;
pub mod graph;
pub mod health_snapshots;
pub mod hooks;
pub mod import;
pub mod issue_groups;
pub mod lessons;
pub mod messages;
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
pub mod themes;
pub mod validation;
pub mod violations;
pub mod watcher;
pub mod workflow;
