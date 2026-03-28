/// Sidecar process manager: spawn, restart, health monitoring, and I/O.
pub mod manager;
/// NDJSON protocol framing and line-delimited message parsing.
pub mod protocol;
/// Sidecar request and response type definitions.
pub mod types;
