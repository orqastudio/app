//! HTTP daemon mode for `orqa-validation`.
//!
//! Starts a `tiny_http` server that exposes all validation functionality as
//! JSON API endpoints. All artifact graph state is loaded once on startup and
//! held in memory. A `POST /reload` endpoint rebuilds state from disk.
//!
//! # Endpoints
//!
//! | Method | Path                     | Description                        |
//! |--------|--------------------------|------------------------------------|
//! | GET    | `/health`                | Server liveness + artifact counts  |
//! | POST   | `/parse`                 | Parse a single artifact file       |
//! | POST   | `/query`                 | Query the artifact graph           |
//! | POST   | `/hook`                  | Evaluate hook lifecycle rules      |
//! | POST   | `/content/agent`         | Load agent preamble                |
//! | POST   | `/content/knowledge`     | Load knowledge artifact            |
//! | POST   | `/content/behavioral`    | Extract behavioral messages        |
//! | POST   | `/validate`              | Full graph validation report       |
//! | POST   | `/traceability`          | Traceability from cached graph     |
//! | POST   | `/reload`                | Rebuild all state from disk        |
//!
//! # PID file
//!
//! The daemon writes its PID to `<project_root>/tmp/daemon.pid` on startup and
//! removes it on shutdown. A second invocation that finds a live PID file exits
//! with an error.

use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use serde::Serialize;
use serde_json::Value;
use tiny_http::{Method, Request, Response, Server, StatusCode};

use crate::content::{extract_behavioral_messages, find_agent, find_knowledge};
use crate::context::build_validation_context_complete;
use crate::error::ValidationError;
use crate::graph::{build_artifact_graph, load_project_config, ArtifactGraph, ArtifactNode};
use crate::metrics::compute_traceability;
use crate::parse::{parse_artifact, passes_filters, passes_search};
use crate::platform::{scan_plugin_manifests, PluginContributions};
use crate::types::{
    AppliedFix, EnforcementEvent, EnforcementResult, GraphHealth, HookContext, HookResult,
    IntegrityCategory, IntegrityCheck, IntegritySeverity, ValidationContext,
};
use crate::{auto_fix, compute_health, evaluate_hook, validate};

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// All pre-loaded validation state held in memory across requests.
struct DaemonState {
    project_root: PathBuf,
    graph: ArtifactGraph,
    ctx: ValidationContext,
    plugin_contributions: PluginContributions,
}

impl DaemonState {
    /// Build daemon state by scanning the project root.
    fn build(project_root: &Path) -> Result<Self, ValidationError> {
        let graph = build_artifact_graph(project_root)?;
        let (valid_statuses, delivery, project_relationships) =
            load_project_config(project_root);
        let plugin_contributions = scan_plugin_manifests(project_root);

        let ctx = build_validation_context_complete(
            &valid_statuses,
            &delivery,
            &project_relationships,
            &plugin_contributions.relationships,
            &plugin_contributions.artifact_types,
            &plugin_contributions.schema_extensions,
            &plugin_contributions.enforcement_mechanisms,
        );

        Ok(Self {
            project_root: project_root.to_path_buf(),
            graph,
            ctx,
            plugin_contributions,
        })
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Start the HTTP daemon and block until a shutdown signal is received.
///
/// Writes a PID file to `<project_root>/tmp/daemon.pid` and removes it on
/// exit. Returns an error if the port is already in use or the PID file
/// indicates another daemon instance is running.
///
/// # Errors
///
/// Returns a boxed error if:
/// - The project root does not exist
/// - A PID file already exists for a running process
/// - The TCP port cannot be bound
/// - Initial state construction fails
pub fn run_daemon(
    project_root: &Path,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    if !project_root.exists() {
        return Err(format!("project root does not exist: {}", project_root.display()).into());
    }

    let pid_path = ensure_tmp_dir(project_root)?;
    check_existing_pid(&pid_path)?;
    write_pid(&pid_path)?;

    // Set up graceful shutdown flag — toggled by signal handler.
    let shutdown = Arc::new(AtomicBool::new(false));
    register_shutdown_handler(Arc::clone(&shutdown));

    let run_result = serve(project_root, port, &shutdown, &pid_path);

    // Always remove PID file, regardless of whether serve() succeeded.
    let _ = std::fs::remove_file(&pid_path);

    run_result
}

// ---------------------------------------------------------------------------
// Server loop
// ---------------------------------------------------------------------------

fn serve(
    project_root: &Path,
    port: u16,
    shutdown: &Arc<AtomicBool>,
    pid_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!(
        "orqa-validation daemon: loading state from {}",
        project_root.display()
    );

    let state = DaemonState::build(project_root)
        .map_err(|e| format!("failed to build initial state: {e}"))?;

    let artifact_count = state.graph.nodes.len();
    let rule_count = state
        .graph
        .nodes
        .values()
        .filter(|n| n.artifact_type == "rule")
        .count();

    let shared = Arc::new(Mutex::new(state));

    let addr = format!("0.0.0.0:{port}");
    let server = Server::http(&addr)
        .map_err(|e| format!("failed to bind {addr}: {e}"))?;

    eprintln!(
        "orqa-validation daemon: listening on http://{addr} ({artifact_count} artifacts, {rule_count} rules)"
    );
    eprintln!("  PID file: {}", pid_path.display());
    eprintln!("  Send SIGTERM or SIGINT to shut down.");

    loop {
        if shutdown.load(Ordering::Relaxed) {
            eprintln!("orqa-validation daemon: shutdown signal received, stopping.");
            break;
        }

        // Non-blocking receive with a short timeout so we can check shutdown.
        match server.recv_timeout(std::time::Duration::from_millis(200)) {
            Ok(Some(request)) => {
                handle_request(request, &shared);
            }
            Ok(None) => {
                // Timeout — loop back to check shutdown flag.
            }
            Err(e) => {
                eprintln!("orqa-validation daemon: recv error: {e}");
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Request dispatch
// ---------------------------------------------------------------------------

fn handle_request(mut request: Request, shared: &Arc<Mutex<DaemonState>>) {
    let method = request.method().clone();
    let url = request.url().to_owned();

    // Read body once.
    let mut body = String::new();
    let _ = request.as_reader().read_to_string(&mut body);

    let result: Result<Value, (u16, String)> = match (method.clone(), url.trim_end_matches('/')) {
        (Method::Get, "/health") => handle_health(shared),
        (Method::Post, "/parse") => handle_parse(&body, shared),
        (Method::Post, "/query") => handle_query(&body, shared),
        (Method::Post, "/hook") => handle_hook(&body, shared),
        (Method::Post, "/content/agent") => handle_content_agent(&body, shared),
        (Method::Post, "/content/knowledge") => handle_content_knowledge(&body, shared),
        (Method::Post, "/content/behavioral") => handle_content_behavioral(shared),
        (Method::Post, "/validate") => handle_validate(&body, shared),
        (Method::Post, "/traceability") => handle_traceability(&body, shared),
        (Method::Post, "/reload") => handle_reload(shared),
        _ => Err((404, format!("not found: {method} {url}"))),
    };

    let (status, body_bytes) = match result {
        Ok(value) => {
            let json = serde_json::to_string_pretty(&value)
                .unwrap_or_else(|e| format!("{{\"error\":\"serialisation failed: {e}\"}}"));
            (200u16, json)
        }
        Err((code, msg)) => {
            let json = serde_json::to_string_pretty(&serde_json::json!({ "error": msg }))
                .unwrap_or_else(|_| format!("{{\"error\":\"{msg}\"}}"));
            (code, json)
        }
    };

    let body_vec = body_bytes.into_bytes();
    let content_length = body_vec.len();
    let response = Response::new(
        StatusCode(status),
        vec![tiny_http::Header::from_bytes(
            b"Content-Type",
            b"application/json",
        )
        .unwrap()],
        Cursor::new(body_vec),
        Some(content_length),
        None,
    );

    if let Err(e) = request.respond(response) {
        eprintln!("orqa-validation daemon: failed to send response: {e}");
    }
}

// ---------------------------------------------------------------------------
// Handler implementations
// ---------------------------------------------------------------------------

/// `GET /health` — returns artifact and rule counts.
fn handle_health(shared: &Arc<Mutex<DaemonState>>) -> Result<Value, (u16, String)> {
    let state = lock(shared)?;
    let artifact_count = state.graph.nodes.len();
    let rule_count = state
        .graph
        .nodes
        .values()
        .filter(|n| n.artifact_type == "rule")
        .count();
    Ok(serde_json::json!({
        "status": "ok",
        "artifacts": artifact_count,
        "rules": rule_count,
    }))
}

/// `POST /parse` — `{ "file": "/abs/path.md" }` → `ParsedArtifact`.
fn handle_parse(
    body: &str,
    shared: &Arc<Mutex<DaemonState>>,
) -> Result<Value, (u16, String)> {
    let req: Value = parse_body(body)?;
    let file = req_str(&req, "file")?;
    let file_path = PathBuf::from(file);

    if !file_path.exists() {
        return Err((404, format!("file does not exist: {file}")));
    }

    let state = lock(shared)?;
    let parsed = parse_artifact(&file_path, &state.project_root)
        .map_err(|e| (422u16, format!("parse error: {e}")))?;
    to_value(parsed)
}

/// `POST /query` — `{ "type": "rule", "status": "active", "id": "RULE-…" }` → `[ArtifactNode]`.
///
/// Returns full `ArtifactNode` objects (including `references_out` and
/// `references_in`) so that callers like `graph_relationships` can read
/// relationship data without a separate lookup.
fn handle_query(
    body: &str,
    shared: &Arc<Mutex<DaemonState>>,
) -> Result<Value, (u16, String)> {
    let req: Value = parse_body(body)?;
    let type_filter = req.get("type").and_then(Value::as_str);
    let status_filter = req.get("status").and_then(Value::as_str);
    let id_filter = req.get("id").and_then(Value::as_str);
    let search_filter = req.get("search").and_then(Value::as_str);

    let state = lock(shared)?;

    // Fast path: exact ID lookup (O(1) vs O(n) scan).
    if let Some(idf) = id_filter {
        if let Some(node) = state.graph.nodes.get(idf) {
            if passes_filters(node, type_filter, status_filter)
                && passes_search(node, search_filter)
            {
                return to_value(vec![node.clone()]);
            }
        }
    }

    let mut results: Vec<ArtifactNode> = state
        .graph
        .nodes
        .iter()
        .filter(|(key, node)| {
            // In organisation mode skip bare-ID alias nodes to avoid duplicates.
            if key.as_str() == node.id && node.project.is_some() {
                return false;
            }
            if !passes_filters(node, type_filter, status_filter) {
                return false;
            }
            if let Some(idf) = id_filter {
                if node.id != idf && !node.id.starts_with(idf) {
                    return false;
                }
            }
            passes_search(node, search_filter)
        })
        .map(|(_, node)| node.clone())
        .collect();

    // Stable output order: sort by artifact type then by ID.
    results.sort_by(|a, b| {
        a.artifact_type
            .cmp(&b.artifact_type)
            .then_with(|| a.id.cmp(&b.id))
    });

    to_value(results)
}

/// `POST /hook` — `HookContext` JSON → `HookResult`.
fn handle_hook(
    body: &str,
    shared: &Arc<Mutex<DaemonState>>,
) -> Result<Value, (u16, String)> {
    let req: Value = parse_body(body)?;
    let event = req
        .get("event")
        .and_then(Value::as_str)
        .ok_or_else(|| (400u16, "missing field: event".to_owned()))?
        .to_owned();

    let ctx = HookContext {
        event,
        tool_name: req
            .get("tool_name")
            .and_then(Value::as_str)
            .map(str::to_owned),
        tool_input: req.get("tool_input").cloned(),
        file_path: req
            .get("file_path")
            .and_then(Value::as_str)
            .map(str::to_owned),
        user_message: req
            .get("user_message")
            .and_then(Value::as_str)
            .map(str::to_owned),
        agent_type: req
            .get("agent_type")
            .and_then(Value::as_str)
            .map(str::to_owned),
    };

    // evaluate_hook rebuilds the graph internally; we use the cached project root.
    let project_root = {
        let state = lock(shared)?;
        state.project_root.clone()
    };

    // For the hook evaluator we re-use the public API which builds its own graph.
    // This is intentional: hook evaluation reads fresh rules from disk so that
    // rule changes take effect immediately even without a /reload call.
    // The overhead is acceptable given hook call frequency.
    let result: HookResult = evaluate_hook(&ctx, &project_root);
    to_value(result)
}

/// `POST /content/agent` — `{ "match": "orchestrator" }` → `AgentContent`.
fn handle_content_agent(
    body: &str,
    shared: &Arc<Mutex<DaemonState>>,
) -> Result<Value, (u16, String)> {
    let req: Value = parse_body(body)?;
    let agent_type = req_str(&req, "match")?;
    let project_root = {
        let state = lock(shared)?;
        state.project_root.clone()
    };
    match find_agent(&project_root, agent_type)
        .map_err(|e| (500u16, format!("agent load error: {e}")))?
    {
        Some(agent) => to_value(agent),
        None => Err((404, format!("no agent found matching: {agent_type}"))),
    }
}

/// `POST /content/knowledge` — `{ "key": "domain-services" }` → `KnowledgeContent`.
fn handle_content_knowledge(
    body: &str,
    shared: &Arc<Mutex<DaemonState>>,
) -> Result<Value, (u16, String)> {
    let req: Value = parse_body(body)?;
    let key = req_str(&req, "key")?;
    let project_root = {
        let state = lock(shared)?;
        state.project_root.clone()
    };
    match find_knowledge(&project_root, key)
        .map_err(|e| (500u16, format!("knowledge load error: {e}")))?
    {
        Some(knowledge) => to_value(knowledge),
        None => Err((404, format!("no knowledge found for key: {key}"))),
    }
}

/// `POST /content/behavioral` — `{}` → `BehavioralMessages`.
fn handle_content_behavioral(
    shared: &Arc<Mutex<DaemonState>>,
) -> Result<Value, (u16, String)> {
    let (graph, project_root) = {
        let state = lock(shared)?;
        (state.graph.clone(), state.project_root.clone())
    };
    let result = extract_behavioral_messages(&graph, &project_root)
        .map_err(|e| (500u16, format!("behavioral messages error: {e}")))?;
    to_value(result)
}

/// Validate subcommand output type — mirrors the CLI `Report`.
#[derive(Serialize)]
struct ValidationReport {
    checks: Vec<IntegrityCheck>,
    health: GraphHealth,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixes_applied: Option<Vec<AppliedFix>>,
    enforcement_events: Vec<EnforcementEvent>,
}

/// `POST /validate` — `{ "fix": false }` → `ValidationReport`.
fn handle_validate(
    body: &str,
    shared: &Arc<Mutex<DaemonState>>,
) -> Result<Value, (u16, String)> {
    let req: Value = parse_body(body)?;
    let apply_fixes = req
        .get("fix")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let (graph, ctx, project_root) = {
        let state = lock(shared)?;
        (
            state.graph.clone(),
            state.ctx.clone(),
            state.project_root.clone(),
        )
    };

    let checks = validate(&graph, &ctx);
    let health = compute_health(&graph);

    let fixes_applied = if apply_fixes {
        Some(
            auto_fix(&graph, &checks, &project_root)
                .map_err(|e| (500u16, format!("auto-fix error: {e}")))?,
        )
    } else {
        None
    };

    let enforcement_events = checks_to_enforcement_events(&checks);

    to_value(ValidationReport {
        checks,
        health,
        fixes_applied,
        enforcement_events,
    })
}

/// `POST /traceability` — `{ "artifact_id": "EPIC-094" }` → `TraceabilityResult`.
///
/// Uses the daemon's cached graph instead of rebuilding from disk.
fn handle_traceability(
    body: &str,
    shared: &Arc<Mutex<DaemonState>>,
) -> Result<Value, (u16, String)> {
    let req: Value = parse_body(body)?;
    let artifact_id = req_str(&req, "artifact_id")?;

    if artifact_id.trim().is_empty() {
        return Err((400, "artifact_id cannot be empty".to_owned()));
    }

    let state = lock(shared)?;
    let result = compute_traceability(&state.graph, artifact_id);
    to_value(result)
}

/// `POST /reload` — rebuild all state from disk.
fn handle_reload(shared: &Arc<Mutex<DaemonState>>) -> Result<Value, (u16, String)> {
    let project_root = {
        let state = lock(shared)?;
        state.project_root.clone()
    };

    let new_state = DaemonState::build(&project_root)
        .map_err(|e| (500u16, format!("reload failed: {e}")))?;

    let artifact_count = new_state.graph.nodes.len();
    let rule_count = new_state
        .graph
        .nodes
        .values()
        .filter(|n| n.artifact_type == "rule")
        .count();

    {
        let mut state = lock(shared)?;
        *state = new_state;
    }

    Ok(serde_json::json!({
        "status": "reloaded",
        "artifacts": artifact_count,
        "rules": rule_count,
    }))
}

// ---------------------------------------------------------------------------
// PID file management
// ---------------------------------------------------------------------------

fn ensure_tmp_dir(project_root: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let tmp = project_root.join("tmp");
    std::fs::create_dir_all(&tmp)
        .map_err(|e| format!("failed to create tmp dir: {e}"))?;
    Ok(tmp.join("daemon.pid"))
}

fn check_existing_pid(pid_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !pid_path.exists() {
        return Ok(());
    }

    let contents = std::fs::read_to_string(pid_path)
        .map_err(|e| format!("failed to read PID file: {e}"))?;
    let pid: u32 = contents
        .trim()
        .parse()
        .map_err(|_| format!("malformed PID file: {}", pid_path.display()))?;

    // On Windows there is no direct "is process alive" API in std. Check via
    // `/proc/<pid>` on Unix; on Windows we attempt to open the process handle.
    if process_is_alive(pid) {
        return Err(format!(
            "daemon already running (PID {pid}). Stop it before starting a new instance.\n\
             PID file: {}",
            pid_path.display()
        )
        .into());
    }

    // Stale PID file — process no longer alive. Remove and continue.
    eprintln!(
        "orqa-validation daemon: removing stale PID file (PID {pid} is not running)"
    );
    std::fs::remove_file(pid_path)
        .map_err(|e| format!("failed to remove stale PID file: {e}"))?;
    Ok(())
}

fn write_pid(pid_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let pid = std::process::id();
    std::fs::write(pid_path, format!("{pid}\n"))
        .map_err(|e| format!("failed to write PID file: {e}"))?;
    Ok(())
}

/// Returns `true` if a process with the given PID is currently running.
///
/// Uses platform-specific checks: `/proc/<pid>` on Linux/macOS, and a
/// `OpenProcess` handle probe on Windows.
#[cfg(unix)]
fn process_is_alive(pid: u32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
}

#[cfg(windows)]
fn process_is_alive(pid: u32) -> bool {
    // SAFETY: OpenProcess with SYNCHRONIZE access only. We immediately close the
    // handle. This is the standard Windows pattern for checking process existence.
    use std::os::windows::raw::HANDLE;
    extern "system" {
        fn OpenProcess(desired_access: u32, inherit_handle: i32, pid: u32) -> HANDLE;
        fn CloseHandle(handle: HANDLE) -> i32;
    }
    const SYNCHRONIZE: u32 = 0x0010_0000;
    unsafe {
        let handle = OpenProcess(SYNCHRONIZE, 0, pid);
        if handle.is_null() {
            return false;
        }
        CloseHandle(handle);
        true
    }
}

#[cfg(not(any(unix, windows)))]
fn process_is_alive(_pid: u32) -> bool {
    // Unknown platform — assume stale.
    false
}

// ---------------------------------------------------------------------------
// Signal handling
// ---------------------------------------------------------------------------

/// Install a Ctrl-C / SIGTERM handler that sets the `shutdown` flag.
///
/// `tiny_http` has no built-in shutdown API; we use a polling loop with a
/// short timeout and an atomic flag. This is the standard pattern for
/// synchronous HTTP servers without an async runtime.
fn register_shutdown_handler(shutdown: Arc<AtomicBool>) {
    // std::thread is used intentionally to avoid an async runtime dependency.
    std::thread::spawn(move || {
        // Block until a Ctrl-C signal arrives.
        // The `ctrlc` crate is not available here, so we use a simple signal
        // handler via the standard library where possible.
        wait_for_signal();
        shutdown.store(true, Ordering::Relaxed);
    });
}

#[cfg(unix)]
fn wait_for_signal() {
    use std::mem::MaybeUninit;
    // SAFETY: sigwait is safe when called on a properly initialised sigset.
    unsafe {
        let mut set = MaybeUninit::<libc_sigset_t>::uninit();
        sigemptyset(set.as_mut_ptr());
        sigaddset(set.as_mut_ptr(), 2); // SIGINT
        sigaddset(set.as_mut_ptr(), 15); // SIGTERM
        sigprocmask(2, set.as_ptr(), std::ptr::null_mut()); // SIG_BLOCK
        let mut sig: i32 = 0;
        sigwait(set.as_ptr(), &mut sig);
    }
}

// Minimal libc types and extern declarations for Unix signal handling.
// We avoid the `libc` crate to keep dependencies minimal.
#[cfg(unix)]
#[repr(C)]
struct libc_sigset_t([u8; 128]);

#[cfg(unix)]
extern "C" {
    fn sigemptyset(set: *mut libc_sigset_t) -> i32;
    fn sigaddset(set: *mut libc_sigset_t, signum: i32) -> i32;
    fn sigprocmask(how: i32, set: *const libc_sigset_t, oldset: *mut libc_sigset_t) -> i32;
    fn sigwait(set: *const libc_sigset_t, sig: *mut i32) -> i32;
}

#[cfg(windows)]
fn wait_for_signal() {
    // On Windows, SetConsoleCtrlHandler is the equivalent. For simplicity we
    // park the thread and rely on the process receiving a CTRL_C_EVENT or
    // CTRL_BREAK_EVENT that the OS delivers as a structured exception.
    // The atomic shutdown flag is checked in the serve loop, so this thread
    // just waits indefinitely — the process will be killed externally or the
    // user will Ctrl-C the console.
    loop {
        std::thread::park();
    }
}

#[cfg(not(any(unix, windows)))]
fn wait_for_signal() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn lock(
    shared: &Arc<Mutex<DaemonState>>,
) -> Result<std::sync::MutexGuard<'_, DaemonState>, (u16, String)> {
    shared
        .lock()
        .map_err(|_| (500u16, "state lock poisoned".to_owned()))
}

fn parse_body(body: &str) -> Result<Value, (u16, String)> {
    if body.trim().is_empty() {
        return Ok(Value::Object(serde_json::Map::new()));
    }
    serde_json::from_str(body)
        .map_err(|e| (400u16, format!("invalid JSON body: {e}")))
}

fn req_str<'a>(req: &'a Value, key: &str) -> Result<&'a str, (u16, String)> {
    req.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| (400u16, format!("missing field: {key}")))
}

fn to_value<T: Serialize>(v: T) -> Result<Value, (u16, String)> {
    serde_json::to_value(v).map_err(|e| (500u16, format!("serialisation error: {e}")))
}

fn checks_to_enforcement_events(checks: &[IntegrityCheck]) -> Vec<EnforcementEvent> {
    checks
        .iter()
        .filter(|c| matches!(c.category, IntegrityCategory::SchemaViolation))
        .map(|c| {
            let result = match c.severity {
                IntegritySeverity::Error => EnforcementResult::Fail,
                IntegritySeverity::Warning => EnforcementResult::Warn,
                IntegritySeverity::Info => EnforcementResult::Pass,
            };
            EnforcementEvent {
                mechanism: "json-schema".to_owned(),
                check_type: "frontmatter".to_owned(),
                rule_id: None,
                artifact_id: Some(c.artifact_id.clone()),
                result,
                message: c.message.clone(),
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_project(tmp: &TempDir) {
        // Minimal .orqa layout so build_artifact_graph doesn't error.
        fs::create_dir_all(tmp.path().join(".orqa/process/rules")).unwrap();
    }

    fn start_test_server(project_root: PathBuf, port: u16) -> Arc<AtomicBool> {
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = Arc::clone(&shutdown);
        std::thread::spawn(move || {
            let _ = serve(&project_root, port, &shutdown_clone, Path::new("/dev/null"));
        });
        // Give the server a moment to bind.
        std::thread::sleep(std::time::Duration::from_millis(100));
        shutdown
    }

    fn get(port: u16, path: &str) -> (u16, Value) {
        let addr = format!("http://127.0.0.1:{port}{path}");
        let response = ureq_get(&addr);
        response
    }

    fn post(port: u16, path: &str, body: &Value) -> (u16, Value) {
        let addr = format!("http://127.0.0.1:{port}{path}");
        ureq_post(&addr, body)
    }

    // Minimal HTTP client using std TcpStream (no ureq dep in tests).
    fn ureq_get(url: &str) -> (u16, Value) {
        http_request("GET", url, "")
    }

    fn ureq_post(url: &str, body: &Value) -> (u16, Value) {
        let body_str = serde_json::to_string(body).unwrap();
        http_request("POST", url, &body_str)
    }

    fn http_request(method: &str, url: &str, body: &str) -> (u16, Value) {
        use std::io::{Read as _, Write as _};
        use std::net::TcpStream;

        // Parse url: http://127.0.0.1:<port><path>
        let without_scheme = url.strip_prefix("http://").unwrap_or(url);
        let (host_port, path) = without_scheme
            .split_once('/')
            .map(|(h, p)| (h, format!("/{p}")))
            .unwrap_or((without_scheme, "/".to_owned()));

        let mut stream = TcpStream::connect(host_port).expect("connect failed");
        let content_length = body.len();
        let request = format!(
            "{method} {path} HTTP/1.1\r\nHost: {host_port}\r\nContent-Type: application/json\r\nContent-Length: {content_length}\r\nConnection: close\r\n\r\n{body}"
        );
        stream.write_all(request.as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        // Parse status line.
        let status_line = response.lines().next().unwrap_or("HTTP/1.1 500");
        let status: u16 = status_line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(500);

        // Body is after the blank line.
        let body_str = response
            .split_once("\r\n\r\n")
            .map(|(_, b)| b)
            .unwrap_or("{}");
        let value: Value = serde_json::from_str(body_str).unwrap_or(Value::Null);

        (status, value)
    }

    #[test]
    fn health_endpoint_returns_ok() {
        let tmp = TempDir::new().unwrap();
        make_project(&tmp);
        let port = 13_101u16;
        let shutdown = start_test_server(tmp.path().to_path_buf(), port);

        let (status, body) = get(port, "/health");
        assert_eq!(status, 200);
        assert_eq!(body["status"], "ok");
        assert!(body["artifacts"].as_u64().is_some());
        assert!(body["rules"].as_u64().is_some());

        shutdown.store(true, Ordering::Relaxed);
    }

    #[test]
    fn unknown_route_returns_404() {
        let tmp = TempDir::new().unwrap();
        make_project(&tmp);
        let port = 13_102u16;
        let shutdown = start_test_server(tmp.path().to_path_buf(), port);

        let (status, body) = get(port, "/nonexistent");
        assert_eq!(status, 404);
        assert!(body["error"].as_str().is_some());

        shutdown.store(true, Ordering::Relaxed);
    }

    #[test]
    fn query_endpoint_returns_array() {
        let tmp = TempDir::new().unwrap();
        make_project(&tmp);
        let port = 13_103u16;
        let shutdown = start_test_server(tmp.path().to_path_buf(), port);

        let (status, body) = post(port, "/query", &serde_json::json!({}));
        assert_eq!(status, 200);
        assert!(body.is_array(), "expected array, got: {body}");

        shutdown.store(true, Ordering::Relaxed);
    }

    #[test]
    fn parse_endpoint_rejects_missing_field() {
        let tmp = TempDir::new().unwrap();
        make_project(&tmp);
        let port = 13_104u16;
        let shutdown = start_test_server(tmp.path().to_path_buf(), port);

        // No "file" field — should be 400.
        let (status, body) = post(port, "/parse", &serde_json::json!({}));
        assert_eq!(status, 400);
        assert!(body["error"].as_str().unwrap().contains("missing field"));

        shutdown.store(true, Ordering::Relaxed);
    }

    #[test]
    fn parse_endpoint_returns_artifact() {
        let tmp = TempDir::new().unwrap();
        make_project(&tmp);

        // Write a minimal artifact to parse.
        let artifact_path = tmp.path().join(".orqa/process/rules/RULE-test1234.md");
        fs::write(
            &artifact_path,
            "---\nid: RULE-test1234\ntitle: Test Rule\nstatus: active\ntype: rule\n---\n\nBody.\n",
        )
        .unwrap();

        let port = 13_105u16;
        let shutdown = start_test_server(tmp.path().to_path_buf(), port);

        let (status, body) = post(
            port,
            "/parse",
            &serde_json::json!({ "file": artifact_path.to_str().unwrap() }),
        );
        assert_eq!(status, 200);
        assert_eq!(body["id"], "RULE-test1234");

        shutdown.store(true, Ordering::Relaxed);
    }

    #[test]
    fn hook_endpoint_allows_clean_command() {
        let tmp = TempDir::new().unwrap();
        make_project(&tmp);
        let port = 13_106u16;
        let shutdown = start_test_server(tmp.path().to_path_buf(), port);

        let (status, body) = post(
            port,
            "/hook",
            &serde_json::json!({
                "event": "PreAction",
                "tool_name": "Bash",
                "tool_input": { "command": "git status" }
            }),
        );
        assert_eq!(status, 200);
        assert_eq!(body["action"], "allow");

        shutdown.store(true, Ordering::Relaxed);
    }

    #[test]
    fn reload_endpoint_rebuilds_state() {
        let tmp = TempDir::new().unwrap();
        make_project(&tmp);
        let port = 13_107u16;
        let shutdown = start_test_server(tmp.path().to_path_buf(), port);

        let (status, body) = post(port, "/reload", &serde_json::json!({}));
        assert_eq!(status, 200);
        assert_eq!(body["status"], "reloaded");
        assert!(body["artifacts"].as_u64().is_some());

        shutdown.store(true, Ordering::Relaxed);
    }

    #[test]
    fn validate_endpoint_returns_report() {
        let tmp = TempDir::new().unwrap();
        make_project(&tmp);
        let port = 13_108u16;
        let shutdown = start_test_server(tmp.path().to_path_buf(), port);

        let (status, body) = post(port, "/validate", &serde_json::json!({}));
        assert_eq!(status, 200);
        assert!(body["checks"].is_array());
        assert!(body["health"].is_object());

        shutdown.store(true, Ordering::Relaxed);
    }
}
