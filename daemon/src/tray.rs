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
    tracing::warn!(
        subsystem = "tray",
        "[tray] fin icon decode failed — using fallback"
    );
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

/// IDs of the interactive menu items, returned by `build_menu` so the event
/// loop can match incoming `MenuEvent`s to the correct action.
struct MenuIds {
    open_app: tray_icon::menu::MenuId,
    open_devtools: tray_icon::menu::MenuId,
    quit: tray_icon::menu::MenuId,
}

/// Build the tray context menu with current LSP and MCP subprocess statuses.
///
/// Menu structure:
///   - "OrqaStudio Daemon" header (disabled, non-interactive)
///   - Separator
///   - "LSP: <status>" — current LSP server status (non-interactive)
///   - "MCP: <status>" — current MCP server status (non-interactive)
///   - Separator
///   - "Open App" — focus or launch OrqaStudio
///   - "Open DevTools" — focus or launch OrqaDev
///   - Separator
///   - "Quit" — triggers graceful daemon shutdown
fn build_menu(statuses: SubprocessStatuses) -> (tray_icon::menu::Menu, MenuIds) {
    use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};

    let menu = Menu::new();

    let header = MenuItem::new("OrqaStudio Daemon", false, None);
    let sep1 = PredefinedMenuItem::separator();
    let lsp_item = MenuItem::new(format!("LSP: {}", status_label(statuses.lsp)), false, None);
    let mcp_item = MenuItem::new(format!("MCP: {}", status_label(statuses.mcp)), false, None);
    let sep2 = PredefinedMenuItem::separator();
    let open_app_item = MenuItem::new("Open App", true, None);
    let open_devtools_item = MenuItem::new("Open DevTools", true, None);
    let sep3 = PredefinedMenuItem::separator();
    let quit_item = MenuItem::new("Quit", true, None);

    let ids = MenuIds {
        open_app: open_app_item.id().clone(),
        open_devtools: open_devtools_item.id().clone(),
        quit: quit_item.id().clone(),
    };

    menu.append(&header).expect("menu append header");
    menu.append(&sep1).expect("menu append sep1");
    menu.append(&lsp_item).expect("menu append lsp");
    menu.append(&mcp_item).expect("menu append mcp");
    menu.append(&sep2).expect("menu append sep2");
    menu.append(&open_app_item).expect("menu append open_app");
    menu.append(&open_devtools_item)
        .expect("menu append open_devtools");
    menu.append(&sep3).expect("menu append sep3");
    menu.append(&quit_item).expect("menu append quit");

    (menu, ids)
}

/// Rebuild the tray context menu when subprocess statuses have changed.
///
/// Updates `tray` with a fresh menu and refreshes all `MenuIds`. No-op when
/// statuses match the last-rendered values. All IDs must be updated together
/// because each `build_menu` call generates new `MenuId` values — stale IDs
/// will never match incoming events.
fn maybe_refresh_menu(
    tray: &tray_icon::TrayIcon,
    current: SubprocessStatuses,
    ids: &mut MenuIds,
    last_lsp: &mut SubprocessStatus,
    last_mcp: &mut SubprocessStatus,
) {
    if current.lsp != *last_lsp || current.mcp != *last_mcp {
        let (new_menu, new_ids) = build_menu(current);
        *ids = new_ids;
        tray.set_menu(Some(Box::new(new_menu)));
        *last_lsp = current.lsp;
        *last_mcp = current.mcp;
    }
}

/// Focus an existing window with `window_title`, or launch `binary_name` if no
/// window is found.
///
/// On Windows, uses `FindWindowW` to locate the window by title and
/// `SetForegroundWindow` to bring it to the front. On other platforms, falls
/// back to spawning the binary (Tauri single-instance handling should focus
/// the existing window).
fn focus_or_launch(binary_name: &str, window_title: &str) {
    if try_focus_window(window_title) {
        tracing::info!(
            subsystem = "tray",
            window = window_title,
            "[tray] focused existing window"
        );
        return;
    }

    // No existing window — launch the binary.
    launch_binary(binary_name, window_title);
}

/// Attempt to find and focus a window by its title.
///
/// Returns `true` if the window was found and focused, `false` otherwise.
#[cfg(windows)]
#[allow(unsafe_code)]
fn try_focus_window(window_title: &str) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        FindWindowW, IsIconic, SetForegroundWindow, ShowWindow, SW_RESTORE,
    };

    let title_wide: Vec<u16> = window_title
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    // Safety: FindWindowW with null class name searches all top-level windows.
    // The wide string is null-terminated and valid for the duration of the call.
    let hwnd = unsafe { FindWindowW(std::ptr::null(), title_wide.as_ptr()) };
    if hwnd.is_null() {
        return false;
    }

    // Safety: hwnd is a valid window handle returned by FindWindowW.
    // Restore if minimised, then bring to foreground.
    unsafe {
        if IsIconic(hwnd) != 0 {
            ShowWindow(hwnd, SW_RESTORE);
        }
        SetForegroundWindow(hwnd);
    }
    true
}

/// On macOS, use AppleScript to activate the application by window title.
#[cfg(target_os = "macos")]
fn try_focus_window(window_title: &str) -> bool {
    // AppleScript: tell the application whose name matches to activate.
    // Tauri window titles match the app name in the menu bar.
    let script = format!(
        r#"tell application "System Events"
            set targetProcess to first process whose frontmost is false and name contains "{window_title}"
            set frontmost of targetProcess to true
        end tell"#
    );
    std::process::Command::new("osascript")
        .args(["-e", &script])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// On Linux, use wmctrl to activate a window by title. Falls back to xdotool.
#[cfg(not(any(windows, target_os = "macos")))]
fn try_focus_window(window_title: &str) -> bool {
    // Try wmctrl first (most reliable for X11 and some Wayland compositors).
    if std::process::Command::new("wmctrl")
        .args(["-a", window_title])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        return true;
    }

    // Fallback: xdotool for X11.
    std::process::Command::new("xdotool")
        .args(["search", "--name", window_title, "windowactivate"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Launch a binary by name, searching sibling directory and PATH.
///
/// The binary is spawned detached so it survives independently of the daemon.
fn launch_binary(binary_name: &str, label: &str) {
    let binary_path = crate::subprocess::SubprocessManager::find_binary(binary_name);

    let Some(bin) = binary_path else {
        tracing::warn!(
            subsystem = "tray",
            binary = binary_name,
            "[tray] binary not found — cannot launch {label}"
        );
        return;
    };

    let mut cmd = std::process::Command::new(&bin);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP
        cmd.creation_flags(0x0000_0008 | 0x0000_0200);
    }

    match cmd.spawn() {
        Ok(_) => tracing::info!(
            subsystem = "tray",
            binary = %bin.display(),
            "[tray] launched {label}"
        ),
        Err(e) => tracing::warn!(
            subsystem = "tray",
            error = %e,
            binary = %bin.display(),
            "[tray] failed to launch {label}"
        ),
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
    ids: &mut MenuIds,
    last_lsp: &mut SubprocessStatus,
    last_mcp: &mut SubprocessStatus,
    subprocess_statuses: &Arc<Mutex<SubprocessStatuses>>,
    shutdown_flag: &Arc<AtomicBool>,
) -> Option<TrayStatus> {
    use tray_icon::{menu::MenuEvent, MouseButton, MouseButtonState, TrayIconEvent};

    // Process right-click context menu events.
    while let Ok(event) = MenuEvent::receiver().try_recv() {
        if event.id == ids.quit {
            tracing::info!(
                subsystem = "tray",
                "[tray] Quit selected — initiating shutdown"
            );
            shutdown_flag.store(true, Ordering::SeqCst);
            return Some(TrayStatus::Exited);
        }
        if event.id == ids.open_app {
            tracing::info!(subsystem = "tray", "[tray] Open App selected");
            focus_or_launch("orqa-studio", "OrqaStudio");
        }
        if event.id == ids.open_devtools {
            tracing::info!(subsystem = "tray", "[tray] Open DevTools selected");
            focus_or_launch("orqa-devtools", "OrqaDev");
        }
    }

    // Process left-click events — focus or launch the app.
    // Respond on ButtonUp to avoid double-firing with a DoubleClick that may follow.
    while let Ok(event) = TrayIconEvent::receiver().try_recv() {
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } = event
        {
            tracing::info!(subsystem = "tray", "[tray] left-click — focusing app");
            focus_or_launch("orqa-studio", "OrqaStudio");
        }
    }

    let current = subprocess_statuses.lock().map(|g| *g).unwrap_or_default();
    maybe_refresh_menu(tray, current, ids, last_lsp, last_mcp);
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
    let (initial_menu, mut ids) = build_menu(initial);

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
            &tray,
            &mut ids,
            &mut last_lsp,
            &mut last_mcp,
            &subprocess_statuses,
            &shutdown_flag,
        ) {
            return status;
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    TrayStatus::Exited
}
