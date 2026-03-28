// System tray integration for the OrqaStudio daemon.
//
// Provides a native system tray icon with a context menu. The tray must run on
// the main OS thread (required on Windows for the GUI message pump). The tokio
// async runtime is therefore spawned on a background thread pool, and this
// module's `run_tray_loop` drives the OS event loop on the main thread.
//
// If tray initialisation fails (e.g., headless server environment without a
// display), the daemon continues in headless mode — all functional subsystems
// (health endpoint, file watchers, LSP) still operate normally.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Tray integration status returned by `run_tray_loop`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayStatus {
    /// The tray event loop exited normally (Quit was selected or shutdown flag was set).
    Exited,
    /// Tray could not be initialised — daemon is operating headless.
    Headless,
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

/// Build the tray context menu.
///
/// Menu structure:
///   - "OrqaStudio Daemon" header (disabled, non-interactive)
///   - Separator
///   - "Open App" — launches the OrqaStudio application
///   - "Quit" — triggers graceful daemon shutdown
fn build_menu() -> (
    tray_icon::menu::Menu,
    tray_icon::menu::MenuId,
    tray_icon::menu::MenuId,
) {
    use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};

    let menu = Menu::new();

    let header = MenuItem::new("OrqaStudio Daemon", false, None);
    let separator = PredefinedMenuItem::separator();
    let open_item = MenuItem::new("Open App", true, None);
    let quit_item = MenuItem::new("Quit", true, None);

    let open_id = open_item.id().clone();
    let quit_id = quit_item.id().clone();

    menu.append(&header).expect("menu append header");
    menu.append(&separator).expect("menu append separator");
    menu.append(&open_item).expect("menu append open");
    menu.append(&quit_item).expect("menu append quit");

    (menu, open_id, quit_id)
}

/// Run the tray event loop on the calling thread (must be the main thread).
///
/// Initialises the tray icon and context menu, then polls for menu events every
/// 50 ms. Handles:
///   - "Quit" menu item: sets the shutdown flag and returns `TrayStatus::Exited`
///   - "Open App" menu item: logs intent (full app-launch logic is a follow-up)
///   - External shutdown flag: returns `TrayStatus::Exited` when set by signal handler
///
/// Returns `TrayStatus::Headless` immediately if tray initialisation fails
/// (e.g., no display server). The daemon continues operating without a tray in
/// that case.
pub fn run_tray_loop(shutdown_flag: Arc<AtomicBool>) -> TrayStatus {
    use tray_icon::TrayIconBuilder;
    use tray_icon::menu::MenuEvent;

    let icon = build_icon();
    let (menu, _open_id, quit_id) = build_menu();

    let _tray = match TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("OrqaStudio Daemon")
        .with_icon(icon)
        .build()
    {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!(
                subsystem = "tray",
                error = %e,
                "[tray] could not initialise system tray — running headless"
            );
            return TrayStatus::Headless;
        }
    };

    tracing::info!(subsystem = "tray", "[tray] system tray active");

    let menu_channel = MenuEvent::receiver();

    loop {
        // Check external shutdown flag first (set by Ctrl-C / SIGTERM handler).
        if shutdown_flag.load(Ordering::SeqCst) {
            break;
        }

        // Drain pending menu events without blocking.
        while let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_id {
                tracing::info!(subsystem = "tray", "[tray] Quit selected — initiating shutdown");
                shutdown_flag.store(true, Ordering::SeqCst);
                return TrayStatus::Exited;
            }
            // "Open App" — log intent; launching the GUI is a follow-up.
            tracing::info!(subsystem = "tray", "[tray] Open App selected");
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    TrayStatus::Exited
}
