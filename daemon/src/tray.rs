// System tray integration for the OrqaStudio daemon.
//
// Provides a native system tray icon with a context menu. The tray must run on
// the main OS thread (required on Windows for the GUI message pump). The tokio
// async runtime is therefore spawned on a background thread pool, and this
// module's `run_tray_loop` drives the OS event loop on the main thread.
//
// On Windows the tray-icon crate requires a running Win32 message pump on the
// thread that created the tray icon. Right-click context menus and all tray
// window-proc events are only delivered when PeekMessage/DispatchMessage is
// called. This module uses a platform-specific pump helper (see `pump_messages`)
// instead of a bare `sleep` so that right-click menus work correctly.
//
// Left-click opens the app directly (same as "Open App" in the context menu).
// The context menu therefore appears only on right-click. This is achieved by
// setting `menu_on_left_click(false)` and handling `TrayIconEvent::Click` with
// `MouseButton::Left` + `MouseButtonState::Up`.
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
/// Updates `tray` with a fresh menu and refreshes `open_id`, `quit_id`,
/// `last_lsp`, and `last_mcp`. No-op when statuses match the last-rendered
/// values. Both IDs must be updated together because each `build_menu` call
/// generates new `MenuId` values — stale IDs will never match incoming events.
fn maybe_refresh_menu(
    tray: &tray_icon::TrayIcon,
    current: SubprocessStatuses,
    open_id: &mut tray_icon::menu::MenuId,
    quit_id: &mut tray_icon::menu::MenuId,
    last_lsp: &mut SubprocessStatus,
    last_mcp: &mut SubprocessStatus,
) {
    if current.lsp != *last_lsp || current.mcp != *last_mcp {
        let (new_menu, new_open_id, new_quit_id) = build_menu(current);
        *open_id = new_open_id;
        *quit_id = new_quit_id;
        tray.set_menu(Some(Box::new(new_menu)));
        *last_lsp = current.lsp;
        *last_mcp = current.mcp;
    }
}

/// Launch the OrqaStudio application in the default browser.
///
/// Uses the platform-native mechanism to open the app URL:
/// - macOS: `open <url>`
/// - Windows: `cmd /c start <url>`
/// - Linux/other: `xdg-open <url>`
///
/// Errors are logged but do not crash the daemon — the tray remains functional
/// even when the app cannot be launched.
fn open_app() {
    use orqa_engine::ports::{resolve_port_base, VITE_PORT_OFFSET};
    let port = resolve_port_base() + VITE_PORT_OFFSET;
    let url = format!("http://localhost:{port}");

    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open").arg(&url).spawn();

    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("cmd")
        .args(["/c", "start", &url])
        .spawn();

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let result = std::process::Command::new("xdg-open").arg(&url).spawn();

    match result {
        Ok(_) => tracing::info!(subsystem = "tray", %url, "[tray] opened app in browser"),
        Err(e) => tracing::warn!(subsystem = "tray", error = %e, %url, "[tray] failed to open app"),
    }
}

/// Pump pending Win32 messages on Windows without blocking.
///
/// On Windows, tray-icon creates a hidden HWND whose window proc handles tray
/// events (right-click menu, clicks, etc.). Those messages are only delivered
/// when the owning thread pumps its Win32 message queue via PeekMessage +
/// DispatchMessage. This function drains all currently queued messages and
/// returns immediately, allowing the caller to sleep briefly and retry.
///
/// On non-Windows platforms this is a no-op — the tray library handles its
/// own event delivery without requiring a message pump from the caller.
#[cfg(windows)]
#[allow(unsafe_code)]
fn pump_messages() {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE,
    };

    // Drain all currently pending Win32 messages. PeekMessage with PM_REMOVE
    // retrieves and removes the message without blocking. We loop until the
    // queue is empty, then return so the caller can sleep briefly.
    loop {
        let mut msg = MSG {
            hwnd: std::ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: windows_sys::Win32::Foundation::POINT { x: 0, y: 0 },
        };
        // Safety: msg is a valid stack-allocated MSG. null hwnd means all
        // messages for this thread; 0/0 filter means all message types.
        // &raw mut is used to avoid the clippy::borrow_as_ptr lint.
        let has_message =
            unsafe { PeekMessageW(&raw mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) };
        if has_message == 0 {
            break;
        }
        // Safety: TranslateMessage and DispatchMessage are safe to call with a
        // valid MSG. Keyboard messages that are not translated are silently
        // ignored by DispatchMessageW.
        unsafe {
            let _ = TranslateMessage(&raw const msg);
            DispatchMessageW(&raw const msg);
        }
    }
}

/// No-op on non-Windows platforms.
///
/// The tray library handles its own event delivery on macOS and Linux without
/// requiring the caller to run a message pump.
#[cfg(not(windows))]
fn pump_messages() {}

/// Process one iteration of pending tray events.
///
/// Drains the menu event channel (right-click items) and the tray icon event
/// channel (left-click), then refreshes the menu if subprocess statuses changed.
/// Returns `Some(TrayStatus::Exited)` when Quit is selected, `None` to continue.
fn process_events(
    tray: &tray_icon::TrayIcon,
    open_id: &mut tray_icon::menu::MenuId,
    quit_id: &mut tray_icon::menu::MenuId,
    last_lsp: &mut SubprocessStatus,
    last_mcp: &mut SubprocessStatus,
    subprocess_statuses: &Arc<Mutex<SubprocessStatuses>>,
    shutdown_flag: &Arc<AtomicBool>,
) -> Option<TrayStatus> {
    use tray_icon::{MouseButton, MouseButtonState, TrayIconEvent, menu::MenuEvent};

    // Process right-click context menu events.
    while let Ok(event) = MenuEvent::receiver().try_recv() {
        if event.id == *quit_id {
            tracing::info!(subsystem = "tray", "[tray] Quit selected — initiating shutdown");
            shutdown_flag.store(true, Ordering::SeqCst);
            return Some(TrayStatus::Exited);
        }
        if event.id == *open_id {
            tracing::info!(subsystem = "tray", "[tray] Open App selected");
            open_app();
        }
    }

    // Process left-click events — open the app directly.
    // Respond on ButtonUp to avoid double-firing with a DoubleClick that may follow.
    while let Ok(event) = TrayIconEvent::receiver().try_recv() {
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } = event
        {
            tracing::info!(subsystem = "tray", "[tray] left-click — opening app");
            open_app();
        }
    }

    let current = subprocess_statuses.lock().map(|g| *g).unwrap_or_default();
    maybe_refresh_menu(tray, current, open_id, quit_id, last_lsp, last_mcp);
    None
}

/// Run the tray event loop on the calling thread (must be the main thread).
///
/// On each iteration: pumps the Win32 message queue (Windows only), processes
/// pending menu and icon events, refreshes the menu if service statuses changed,
/// then sleeps 50 ms. Returns `TrayStatus::Headless` if initialisation fails.
pub fn run_tray_loop(
    shutdown_flag: Arc<AtomicBool>,
    subprocess_statuses: Arc<Mutex<SubprocessStatuses>>,
) -> TrayStatus {
    use tray_icon::TrayIconBuilder;

    let icon = build_icon();
    let initial = SubprocessStatuses::default();
    let (initial_menu, mut open_id, mut quit_id) = build_menu(initial);

    let tray = match TrayIconBuilder::new()
        .with_menu(Box::new(initial_menu))
        .with_menu_on_left_click(false)
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

    let (mut last_lsp, mut last_mcp) = (initial.lsp, initial.mcp);

    loop {
        if shutdown_flag.load(Ordering::SeqCst) {
            break;
        }

        // Pump the Win32 message queue so tray events reach the hidden HWND
        // window proc. Without this, right-click menus never appear on Windows.
        pump_messages();

        if let Some(status) = process_events(
            &tray, &mut open_id, &mut quit_id,
            &mut last_lsp, &mut last_mcp,
            &subprocess_statuses, &shutdown_flag,
        ) {
            return status;
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    TrayStatus::Exited
}
