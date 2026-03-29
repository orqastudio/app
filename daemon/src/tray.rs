// System tray integration for the OrqaStudio daemon.
//
// Provides a native system tray icon with a context menu. The tray must run on
// the main OS thread (required on Windows for the GUI message pump). The tokio
// async runtime is therefore spawned on a background thread pool, and this
// module's `run_tray_loop` drives the OS event loop on the main thread.
//
// The tray menu is rebuilt on every polling cycle so that LSP and MCP status
// items remain current as subprocesses start, stop, or crash. Status is
// shared from the background event loop via an `Arc<Mutex<SubprocessStatuses>>`.
//
// If tray initialisation fails (e.g., headless server environment without a
// display), the daemon continues in headless mode — all functional subsystems
// (health endpoint, file watchers, LSP, MCP) still operate normally.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::subprocess::SubprocessStatus;

/// Tray integration status returned by `run_tray_loop`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayStatus {
    /// The tray event loop exited normally (Quit was selected or shutdown flag was set).
    Exited,
    /// Tray could not be initialised — daemon is operating headless.
    Headless,
}

/// Subprocess statuses shared between the background event loop and the tray thread.
///
/// Updated by `run_event_loop` every 250 ms polling cycle. Read by the tray
/// loop every 50 ms to rebuild the menu with current LSP and MCP status.
#[derive(Debug, Clone, Copy)]
pub struct SubprocessStatuses {
    /// Last known status of the LSP server subprocess.
    pub lsp: SubprocessStatus,
    /// Last known status of the MCP server subprocess.
    pub mcp: SubprocessStatus,
}

impl Default for SubprocessStatuses {
    /// Default status before any subprocesses have been started.
    fn default() -> Self {
        Self {
            lsp: SubprocessStatus::Stopped,
            mcp: SubprocessStatus::Stopped,
        }
    }
}

/// The OrqaStudio fin icon, embedded at compile time from the brand assets.
const FIN_ICON_PNG: &[u8] = include_bytes!("../../libs/brand/assets/icons/favicon-32x32.png");

/// Build the tray icon from the embedded OrqaStudio fin PNG.
///
/// Decodes the 32x32 fin icon at runtime from the compile-time-embedded PNG
/// bytes. Falls back to a simple green circle if PNG decoding fails.
fn build_icon() -> tray_icon::Icon {
    use image::GenericImageView;

    if let Ok(img) = image::load_from_memory(FIN_ICON_PNG) {
        let (w, h) = img.dimensions();
        let rgba = img.into_rgba8().into_raw();
        if let Ok(icon) = tray_icon::Icon::from_rgba(rgba, w, h) {
            return icon;
        }
    }

    // Fallback: 32x32 cyan circle if PNG decode fails.
    tracing::warn!(subsystem = "tray", "[tray] fin icon decode failed — using fallback");
    let size: u32 = 32;
    let mut rgba = Vec::with_capacity((size * size * 4) as usize);
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - 16.0;
            let dy = y as f32 - 16.0;
            if dx * dx + dy * dy <= 196.0 {
                rgba.extend_from_slice(&[0x45, 0xd6, 0xe9, 0xff]);
            } else {
                rgba.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
            }
        }
    }
    tray_icon::Icon::from_rgba(rgba, size, size)
        .expect("fallback RGBA data is always valid for 32x32 icon")
}

/// Format a `SubprocessStatus` as a human-readable label for the tray menu.
///
/// Returns a short status string suitable for display next to the service name.
fn status_label(status: SubprocessStatus) -> &'static str {
    match status {
        SubprocessStatus::Running => "running",
        SubprocessStatus::Stopped => "stopped",
        SubprocessStatus::Crashed => "crashed",
        SubprocessStatus::BinaryNotFound => "not found",
    }
}

/// Build the tray context menu with current LSP and MCP subprocess statuses.
///
/// Menu structure:
///   - "OrqaStudio Daemon" header (disabled, non-interactive)
///   - Separator
///   - "LSP: <status>" — current LSP server status (non-interactive)
///   - "MCP: <status>" — current MCP server status (non-interactive)
///   - Separator
///   - "Open App" — launches the OrqaStudio application
///   - "Quit" — triggers graceful daemon shutdown
fn build_menu(statuses: SubprocessStatuses) -> (
    tray_icon::menu::Menu,
    tray_icon::menu::MenuId,
    tray_icon::menu::MenuId,
) {
    use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};

    let menu = Menu::new();

    let header = MenuItem::new("OrqaStudio Daemon", false, None);
    let sep1 = PredefinedMenuItem::separator();
    let lsp_item = MenuItem::new(
        format!("LSP: {}", status_label(statuses.lsp)),
        false,
        None,
    );
    let mcp_item = MenuItem::new(
        format!("MCP: {}", status_label(statuses.mcp)),
        false,
        None,
    );
    let sep2 = PredefinedMenuItem::separator();
    let open_item = MenuItem::new("Open App", true, None);
    let quit_item = MenuItem::new("Quit", true, None);

    let open_id = open_item.id().clone();
    let quit_id = quit_item.id().clone();

    menu.append(&header).expect("menu append header");
    menu.append(&sep1).expect("menu append sep1");
    menu.append(&lsp_item).expect("menu append lsp");
    menu.append(&mcp_item).expect("menu append mcp");
    menu.append(&sep2).expect("menu append sep2");
    menu.append(&open_item).expect("menu append open");
    menu.append(&quit_item).expect("menu append quit");

    (menu, open_id, quit_id)
}

/// Rebuild the tray context menu when subprocess statuses have changed.
///
/// Updates `tray` with a fresh menu and refreshes `quit_id`, `last_lsp`, and
/// `last_mcp`. No-op when statuses match the last-rendered values.
fn maybe_refresh_menu(
    tray: &tray_icon::TrayIcon,
    current: SubprocessStatuses,
    quit_id: &mut tray_icon::menu::MenuId,
    last_lsp: &mut SubprocessStatus,
    last_mcp: &mut SubprocessStatus,
) {
    if current.lsp != *last_lsp || current.mcp != *last_mcp {
        let (new_menu, _open_id, new_quit_id) = build_menu(current);
        *quit_id = new_quit_id;
        tray.set_menu(Some(Box::new(new_menu)));
        *last_lsp = current.lsp;
        *last_mcp = current.mcp;
    }
}

/// Run the tray event loop on the calling thread (must be the main thread).
///
/// Polls for menu events every 50 ms. Rebuilds the menu whenever LSP or MCP
/// subprocess statuses change so the tray reflects current state. Returns
/// `TrayStatus::Headless` if tray initialisation fails.
pub fn run_tray_loop(
    shutdown_flag: Arc<AtomicBool>,
    subprocess_statuses: Arc<Mutex<SubprocessStatuses>>,
) -> TrayStatus {
    use tray_icon::TrayIconBuilder;
    use tray_icon::menu::MenuEvent;

    let icon = build_icon();
    let initial = SubprocessStatuses::default();
    let (initial_menu, _open_id, mut quit_id) = build_menu(initial);

    let tray = match TrayIconBuilder::new()
        .with_menu(Box::new(initial_menu))
        .with_tooltip("OrqaStudio Daemon")
        .with_icon(icon)
        .build()
    {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!(subsystem = "tray", error = %e, "[tray] could not initialise system tray — running headless");
            return TrayStatus::Headless;
        }
    };

    tracing::info!(subsystem = "tray", "[tray] system tray active");

    let menu_channel = MenuEvent::receiver();
    let (mut last_lsp, mut last_mcp) = (initial.lsp, initial.mcp);

    loop {
        if shutdown_flag.load(Ordering::SeqCst) {
            break;
        }
        let current = subprocess_statuses.lock().map(|g| *g).unwrap_or_default();
        maybe_refresh_menu(&tray, current, &mut quit_id, &mut last_lsp, &mut last_mcp);

        while let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_id {
                tracing::info!(subsystem = "tray", "[tray] Quit selected — initiating shutdown");
                shutdown_flag.store(true, Ordering::SeqCst);
                return TrayStatus::Exited;
            }
            tracing::info!(subsystem = "tray", "[tray] Open App selected");
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    TrayStatus::Exited
}
