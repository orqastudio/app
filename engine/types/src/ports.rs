// Port constants for all OrqaStudio services.
//
// infrastructure/ports.json is the single source of truth for all port
// assignments. This module embeds that file at compile time (include_str!) and
// parses it once at startup via OnceLock so every crate that needs a port
// imports from here rather than duplicating constants or hardcoding values.
//
// Services with an "offset" in ports.json derive their port from the base plus
// that offset. ORQA_PORT_BASE overrides the runtime base while keeping all
// offsets constant. Services without an offset (forgejo_http, forgejo_ssh) have
// a fixed port that is not affected by ORQA_PORT_BASE.

use std::sync::OnceLock;

use serde::Deserialize;

// Embed the ports JSON at compile time. The path is relative to this source
// file (engine/types/src/ports.rs) → three levels up to the repo root, then
// infrastructure/ports.json.
const PORTS_JSON_STR: &str = include_str!("../../../infrastructure/ports.json");

// ---------------------------------------------------------------------------
// Deserialization types
// ---------------------------------------------------------------------------

/// One entry in the "services" map of infrastructure/ports.json.
#[derive(Debug, Deserialize)]
struct ServiceEntry {
    /// Offset from the base port, or null for fixed-port services.
    offset: Option<u16>,
    /// The absolute port number (base + offset, or fixed value).
    port: u16,
}

/// Top-level shape of infrastructure/ports.json.
#[derive(Debug, Deserialize)]
struct PortsJson {
    /// The base port number (default 10100). Offset-based services derive their
    /// port from base + offset. Tests assert this value for regression safety.
    #[cfg_attr(not(test), allow(dead_code))]
    base: u16,
    services: std::collections::HashMap<String, ServiceEntry>,
}

// ---------------------------------------------------------------------------
// Compile-time-embedded, runtime-parsed singleton
// ---------------------------------------------------------------------------

/// Parsed ports.json. Initialised once on first access via `ports()`.
static PORTS: OnceLock<PortsJson> = OnceLock::new();

/// Return a reference to the parsed ports.json singleton.
///
/// Panics if the embedded JSON is malformed — this would be a compile-time
/// defect caught in CI, not a runtime error in production.
fn ports() -> &'static PortsJson {
    PORTS.get_or_init(|| {
        serde_json::from_str(PORTS_JSON_STR)
            .expect("infrastructure/ports.json is embedded at compile time and must be valid JSON")
    })
}

// ---------------------------------------------------------------------------
// Public constants
// ---------------------------------------------------------------------------

/// Default base port for all OrqaStudio services (daemon offset = 0).
///
/// Equals ports.json `base`. ORQA_PORT_BASE overrides this at runtime.
pub const DEFAULT_PORT_BASE: u16 = 10100;

/// Port offset for the daemon health endpoint.
pub const DAEMON_PORT_OFFSET: u16 = 0;

/// Port offset for the LSP server.
pub const LSP_PORT_OFFSET: u16 = 1;

/// Port offset for the MCP server.
pub const MCP_PORT_OFFSET: u16 = 2;

/// Port offset for the Vite dev server. 10100 + 320 = 10420.
pub const VITE_PORT_OFFSET: u16 = 320;

/// Port offset for the debug dashboard.
pub const DASHBOARD_PORT_OFFSET: u16 = 30;

// ---------------------------------------------------------------------------
// Runtime port resolution
// ---------------------------------------------------------------------------

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

/// Resolve the port for a named service from infrastructure/ports.json.
///
/// For offset-based services: returns resolve_port_base() + offset.
/// For fixed-port services (forgejo_http, forgejo_ssh): returns the fixed port,
/// ignoring ORQA_PORT_BASE since those services cannot be remapped.
/// Returns None if the service name is not found in ports.json.
pub fn resolve_port(service: &str) -> Option<u16> {
    let p = ports();
    let entry = p.services.get(service)?;
    match entry.offset {
        Some(offset) => Some(resolve_port_base() + offset),
        None => Some(entry.port),
    }
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
    fn vite_port_offset_is_320() {
        // 10100 + 320 = 10420, the correct app Vite dev server port.
        assert_eq!(VITE_PORT_OFFSET, 320);
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
    fn ports_json_embeds_and_parses() {
        // Verify the embedded JSON parses correctly and contains expected services.
        let p = ports();
        assert_eq!(p.base, 10100);
        assert!(p.services.contains_key("daemon"));
        assert!(p.services.contains_key("lsp"));
        assert!(p.services.contains_key("mcp"));
        assert!(p.services.contains_key("vite"));
        assert!(p.services.contains_key("forgejo_http"));
        assert!(p.services.contains_key("forgejo_ssh"));
    }

    #[test]
    fn resolve_port_daemon() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("ORQA_PORT_BASE");
        assert_eq!(resolve_port("daemon"), Some(10100));
    }

    #[test]
    fn resolve_port_lsp() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("ORQA_PORT_BASE");
        assert_eq!(resolve_port("lsp"), Some(10101));
    }

    #[test]
    fn resolve_port_mcp() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("ORQA_PORT_BASE");
        assert_eq!(resolve_port("mcp"), Some(10102));
    }

    #[test]
    fn resolve_port_vite_is_10420() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("ORQA_PORT_BASE");
        assert_eq!(resolve_port("vite"), Some(10420));
    }

    #[test]
    fn resolve_port_forgejo_http_is_fixed() {
        let _guard = ENV_MUTEX.lock().unwrap();
        // Fixed port services are not affected by ORQA_PORT_BASE.
        std::env::set_var("ORQA_PORT_BASE", "20000");
        assert_eq!(resolve_port("forgejo_http"), Some(10030));
        std::env::remove_var("ORQA_PORT_BASE");
    }

    #[test]
    fn resolve_port_forgejo_ssh_is_fixed() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::set_var("ORQA_PORT_BASE", "20000");
        assert_eq!(resolve_port("forgejo_ssh"), Some(10222));
        std::env::remove_var("ORQA_PORT_BASE");
    }

    #[test]
    fn resolve_port_unknown_returns_none() {
        assert_eq!(resolve_port("unknown_service"), None);
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
}
