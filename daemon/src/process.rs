//! Process management for the OrqaStudio daemon.
//!
//! Handles PID file lifecycle and project root discovery. The PID file at
//! `.state/daemon.pid` ensures only one daemon instance runs per project at a
//! time. Project root discovery walks up from CWD until it finds the `.orqa/`
//! marker directory.
//!
//! Windows process-liveness checks require unsafe FFI to kernel32 (`OpenProcess`,
//! `CloseHandle`). These are the only `unsafe` operations in this module.
#![allow(unsafe_code)]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Name of the directory that marks an OrqaStudio project root.
const ORQA_DIR: &str = ".orqa";

/// Relative path to the PID file within the project root.
const PID_FILE_PATH: &str = ".state/daemon.pid";

/// Walk up from `start` until a directory containing `.orqa/` is found.
///
/// Returns the project root directory, or an error if no project root is found
/// before the filesystem root. This prevents the daemon from starting outside
/// an OrqaStudio project.
pub fn find_project_root(start: &Path) -> io::Result<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(ORQA_DIR).is_dir() {
            return Ok(current);
        }
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "no OrqaStudio project found (no {} directory in ancestors of {})",
                        ORQA_DIR,
                        start.display()
                    ),
                ))
            }
        }
    }
}

/// Read the PID from `.state/daemon.pid` in the project root and check whether
/// that process is currently running.
///
/// Returns `Ok(true)` if an existing live daemon is found. Returns `Ok(false)`
/// if there is no PID file or the recorded process is no longer running.
/// Returns an error only if the file exists but cannot be parsed.
pub fn check_existing(project_root: &Path) -> io::Result<bool> {
    let pid_path = project_root.join(PID_FILE_PATH);

    if !pid_path.exists() {
        return Ok(false);
    }

    let contents = fs::read_to_string(&pid_path)?;
    let pid_str = contents.trim();
    let pid: u32 = pid_str.parse().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("malformed PID file: {pid_str:?}"),
        )
    })?;

    Ok(process_is_alive(pid))
}

/// Write the current process PID to `.state/daemon.pid` in the project root.
///
/// Creates `.state/` if it does not already exist. Overwrites any existing PID
/// file — the caller must have already verified no live daemon is running via
/// [`check_existing`].
pub fn write_pid(project_root: &Path) -> io::Result<()> {
    let state_dir = project_root.join(".state");
    fs::create_dir_all(&state_dir)?;

    let pid_path = state_dir.join("daemon.pid");
    let pid = std::process::id();
    fs::write(&pid_path, pid.to_string())?;
    Ok(())
}

/// Remove the PID file on graceful shutdown.
///
/// Silently ignores `NotFound` errors — the file may have been deleted by a
/// previous cleanup pass. Other errors are returned to the caller.
pub fn cleanup_pid(project_root: &Path) -> io::Result<()> {
    let pid_path = project_root.join(PID_FILE_PATH);
    match fs::remove_file(&pid_path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

/// Return true if a process with the given PID is currently alive.
///
/// On Windows, uses `OpenProcess` via a cross-platform compatibility check.
/// On Unix-like systems (not the primary target), sends signal 0.
/// This is a best-effort check — race conditions between check and start are
/// acceptable for PID file semantics.
#[cfg(target_os = "windows")]
fn process_is_alive(pid: u32) -> bool {
    use std::os::windows::io::RawHandle;
    // SAFETY: OpenProcess with PROCESS_QUERY_LIMITED_INFORMATION (0x1000) and
    // CloseHandle are straightforward Windows API calls. We only use the handle
    // to determine liveness and immediately close it.
    unsafe {
        let handle: RawHandle = windows_process_open(pid);
        if handle.is_null() {
            return false;
        }
        windows_process_close(handle);
        true
    }
}

#[cfg(target_os = "windows")]
unsafe fn windows_process_open(pid: u32) -> *mut std::ffi::c_void {
    // Link to kernel32 OpenProcess/CloseHandle via raw extern declarations.
    // PROCESS_QUERY_LIMITED_INFORMATION = 0x1000
    extern "system" {
        fn OpenProcess(
            dwDesiredAccess: u32,
            bInheritHandle: i32,
            dwProcessId: u32,
        ) -> *mut std::ffi::c_void;
    }
    OpenProcess(0x1000, 0, pid)
}

#[cfg(target_os = "windows")]
unsafe fn windows_process_close(handle: *mut std::ffi::c_void) {
    extern "system" {
        fn CloseHandle(hObject: *mut std::ffi::c_void) -> i32;
    }
    CloseHandle(handle);
}

#[cfg(not(target_os = "windows"))]
fn process_is_alive(pid: u32) -> bool {
    // On Unix, signal 0 checks whether the process exists without sending a
    // real signal.
    let pid_i = pid as libc::pid_t;
    unsafe { libc::kill(pid_i, 0) == 0 }
}
