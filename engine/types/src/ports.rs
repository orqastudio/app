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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Mutex to serialize tests that mutate ORQA_PORT_BASE env var.
    /// Without this, parallel test execution causes race conditions.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn default_port_base_is_10100() {
        assert_eq!(DEFAULT_PORT_BASE, 10100);
    }

    #[test]
    fn daemon_port_offset_is_zero() {
        // The daemon is the base service — no offset.
        assert_eq!(DAEMON_PORT_OFFSET, 0);
    }

    #[test]
    fn offsets_are_distinct() {
        // Each service must have a unique port offset.
        let offsets = [
            DAEMON_PORT_OFFSET,
            LSP_PORT_OFFSET,
            MCP_PORT_OFFSET,
            VITE_PORT_OFFSET,
            DASHBOARD_PORT_OFFSET,
            IPC_SOCKET_PORT_OFFSET,
        ];
        for i in 0..offsets.len() {
            for j in (i + 1)..offsets.len() {
                assert_ne!(
                    offsets[i], offsets[j],
                    "offset collision at indices {i} and {j}"
                );
            }
        }
    }

    #[test]
    fn resolve_port_base_returns_default_when_env_absent() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("ORQA_PORT_BASE");
        let base = resolve_port_base();
        assert_eq!(base, DEFAULT_PORT_BASE);
    }

    #[test]
    fn resolve_port_base_reads_env_var() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::set_var("ORQA_PORT_BASE", "20000");
        let base = resolve_port_base();
        std::env::remove_var("ORQA_PORT_BASE");
        assert_eq!(base, 20000);
    }

    #[test]
    fn resolve_port_base_ignores_invalid_env_var() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::set_var("ORQA_PORT_BASE", "not-a-number");
        let base = resolve_port_base();
        std::env::remove_var("ORQA_PORT_BASE");
        assert_eq!(base, DEFAULT_PORT_BASE);
    }

    #[test]
    fn resolve_daemon_port_equals_base_plus_offset() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("ORQA_PORT_BASE");
        let daemon = resolve_daemon_port();
        assert_eq!(daemon, DEFAULT_PORT_BASE + DAEMON_PORT_OFFSET);
    }

    #[test]
    fn resolve_lsp_port_equals_base_plus_offset() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("ORQA_PORT_BASE");
        let lsp = resolve_lsp_port();
        assert_eq!(lsp, DEFAULT_PORT_BASE + LSP_PORT_OFFSET);
    }

    #[test]
    fn resolve_mcp_port_equals_base_plus_offset() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("ORQA_PORT_BASE");
        let mcp = resolve_mcp_port();
        assert_eq!(mcp, DEFAULT_PORT_BASE + MCP_PORT_OFFSET);
    }

    #[test]
    fn resolve_ipc_socket_port_equals_base_plus_offset() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("ORQA_PORT_BASE");
        let ipc = resolve_ipc_socket_port();
        assert_eq!(ipc, DEFAULT_PORT_BASE + IPC_SOCKET_PORT_OFFSET);
    }
}
