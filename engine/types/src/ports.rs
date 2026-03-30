// Port constants for all OrqaStudio services.
//
// All services derive their ports from a single base (ORQA_PORT_BASE, default
// 10100) plus a fixed offset. This module is the single source of truth for
// Rust port numbers — every crate that needs a port imports from here rather
// than duplicating constants inline.

/// Default base port for all OrqaStudio services.
pub const DEFAULT_PORT_BASE: u16 = 10100;

/// Port offset for the daemon health endpoint.
pub const DAEMON_PORT_OFFSET: u16 = 0;

/// Port offset for the LSP server.
pub const LSP_PORT_OFFSET: u16 = 1;

/// Port offset for the MCP server.
pub const MCP_PORT_OFFSET: u16 = 2;

/// Port offset for the Vite dev server (frontend).
pub const VITE_PORT_OFFSET: u16 = 20;

/// Port offset for the debug dashboard.
pub const DASHBOARD_PORT_OFFSET: u16 = 30;

/// Port offset for the IPC socket used by the Tauri app for internal
/// communication between the sidecar processes and the main app process.
pub const IPC_SOCKET_PORT_OFFSET: u16 = 58;

/// Resolve the port base from the ORQA_PORT_BASE environment variable.
///
/// Returns DEFAULT_PORT_BASE when the variable is absent or not a valid u16.
pub fn resolve_port_base() -> u16 {
    std::env::var("ORQA_PORT_BASE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PORT_BASE)
}

/// Resolve the daemon port (port base + DAEMON_PORT_OFFSET).
pub fn resolve_daemon_port() -> u16 {
    resolve_port_base() + DAEMON_PORT_OFFSET
}

/// Resolve the LSP server port (port base + LSP_PORT_OFFSET).
pub fn resolve_lsp_port() -> u16 {
    resolve_port_base() + LSP_PORT_OFFSET
}

/// Resolve the MCP server port (port base + MCP_PORT_OFFSET).
pub fn resolve_mcp_port() -> u16 {
    resolve_port_base() + MCP_PORT_OFFSET
}

/// Resolve the IPC socket port (port base + IPC_SOCKET_PORT_OFFSET).
pub fn resolve_ipc_socket_port() -> u16 {
    resolve_port_base() + IPC_SOCKET_PORT_OFFSET
}
