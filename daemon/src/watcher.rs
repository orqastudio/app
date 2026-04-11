// File watcher registry for OrqaStudio daemon.
//
// Builds a manifest-driven watcher registry at startup by scanning all installed
// plugins. Each enforcement-generator plugin declares one or more `watch` blocks
// in its manifest; the registry registers OS file watches for every declared path
// pattern and invokes the matching generator script when a watched file changes.
//
// In addition to plugin-declared watches, the registry always maintains two
// infrastructure watches that are engine capabilities, not plugin definitions:
//
//   - `.orqa/`    — artifact changes trigger graph rebuild and enforcement reload
//   - `plugins/`  — plugin source changes trigger plugin discovery reload
//
// Events are debounced with a 500 ms window so that rapid successive writes
// (e.g. editor autosave bursts) produce a single notification rather than a
// flood. Paths under `.git/`, `node_modules/`, `target/`, and `.state/` are
// silently ignored to avoid spurious events during build or tooling activity.
//
// When a file-change event fires the registry:
//   1. Checks whether the changed path matches any registered watch_paths pattern.
//   2. For each matching WatchRegistration: spawns the generator as a subprocess.
//   3. Also checks whether the path falls under the infrastructure watch dirs and
//      triggers graph rebuild / enforcement reload / plugin reload accordingly.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, RecommendedCache};
use tracing::{debug, info, warn};

use orqa_engine::plugin::discovery::scan_plugins;
use orqa_engine::plugin::manifest::read_manifest;

use crate::graph_state::GraphState;

/// Infrastructure directories to always watch, relative to the project root.
///
/// These are engine capabilities, not plugin definitions, so they remain
/// hardcoded rather than being declared by any plugin manifest.
const INFRASTRUCTURE_WATCH_DIRS: &[&str] = &[".orqa", "plugins"];

/// Path components that trigger event filtering. Changes under these directories
/// are ignored regardless of where in the tree they appear.
const IGNORED_COMPONENTS: &[&str] = &[".git", "node_modules", "target", ".state"];

/// Debounce window for file-system events.
const DEBOUNCE_MS: u64 = 500;

/// A single manifest-declared file-watch registration from an enforcement generator.
///
/// Created by reading the `enforcement[].watch` block from a plugin manifest.
/// The daemon uses these registrations to decide which generator to invoke when a
/// file changes within a declared watch path.
pub struct WatchRegistration {
    /// The plugin that registered this watcher (e.g. `@orqastudio/plugin-typescript`).
    pub plugin_name: String,
    /// The engine name (e.g. `"eslint"`) — used in log messages.
    pub engine: String,
    /// Glob patterns to watch, relative to the project root.
    pub watch_paths: Vec<String>,
    /// Optional YAML frontmatter filter. Passed to the generator as `--filter`
    /// so it can pre-filter which rule files to include. Not evaluated here.
    pub filter: Option<String>,
    /// Action to perform when a watched path changes.
    pub on_change: WatchAction,
    /// Absolute path to the generator script/binary to invoke.
    pub generator_path: PathBuf,
    /// Absolute path where the generator writes the produced config file.
    pub config_output: PathBuf,
}

/// Action to perform when a watched file changes.
pub enum WatchAction {
    /// Re-run the generator to regenerate the config output.
    Regenerate,
}

/// RAII handle that keeps the debounced watcher alive.
///
/// Dropping this value unregisters all OS watches and stops the background
/// event thread. Create via [`start_watcher`].
pub struct WatchHandle {
    /// The underlying debouncer — kept alive by this field.
    _debouncer: Debouncer<notify::RecommendedWatcher, RecommendedCache>,
}

/// Convert a single enforcement declaration from a plugin into a WatchRegistration.
///
/// Returns `None` and emits a warning if any required field is missing or the
/// `on_change` value is unrecognised.
fn declaration_to_registration(
    plugin_name: &str,
    plugin_dir: &Path,
    project_root: &Path,
    decl: &orqa_engine::plugin::manifest::EnforcementDeclaration,
) -> Option<WatchRegistration> {
    if decl.role != "generator" {
        return None;
    }
    let Some(watch) = &decl.watch else {
        return None;
    };
    let Some(engine) = &decl.engine else {
        warn!(subsystem = "watcher", plugin = %plugin_name,
            "[watcher] generator declaration missing engine field — skipping");
        return None;
    };
    let Some(generator_rel) = &decl.generator else {
        warn!(subsystem = "watcher", plugin = %plugin_name, engine = %engine,
            "[watcher] generator declaration missing generator field — skipping");
        return None;
    };
    let Some(config_output_rel) = &decl.config_output else {
        warn!(subsystem = "watcher", plugin = %plugin_name, engine = %engine,
            "[watcher] generator declaration missing config_output field — skipping");
        return None;
    };
    let on_change = match watch.on_change.as_str() {
        "regenerate" => WatchAction::Regenerate,
        other => {
            warn!(subsystem = "watcher", plugin = %plugin_name, engine = %engine,
                on_change = %other, "[watcher] unknown on_change action — skipping");
            return None;
        }
    };
    Some(WatchRegistration {
        plugin_name: plugin_name.to_owned(),
        engine: engine.clone(),
        watch_paths: watch.paths.clone(),
        filter: watch.filter.clone(),
        on_change,
        generator_path: plugin_dir.join(generator_rel),
        config_output: project_root.join(config_output_rel),
    })
}

/// Build the watcher registry by scanning all installed plugins.
///
/// For each installed plugin that declares `enforcement` entries with
/// `role: "generator"` and a non-null `watch` block, creates a
/// `WatchRegistration` describing what paths to watch and which generator to
/// invoke on change.
fn build_registry(project_root: &Path) -> Vec<WatchRegistration> {
    let plugins = scan_plugins(project_root);
    let mut registrations = Vec::new();

    for plugin in &plugins {
        let plugin_dir = PathBuf::from(&plugin.path);

        let manifest = match read_manifest(&plugin_dir) {
            Ok(m) => m,
            Err(e) => {
                warn!(
                    subsystem = "watcher",
                    plugin = %plugin.name,
                    error = %e,
                    "[watcher] could not read manifest for plugin — skipping"
                );
                continue;
            }
        };

        for decl in &manifest.enforcement {
            if let Some(reg) =
                declaration_to_registration(&plugin.name, &plugin_dir, project_root, decl)
            {
                info!(
                    subsystem = "watcher",
                    plugin = %reg.plugin_name,
                    engine = %reg.engine,
                    paths = ?reg.watch_paths,
                    "[watcher] registered enforcement watcher"
                );
                registrations.push(reg);
            }
        }
    }

    registrations
}

/// Start the manifest-driven file watcher registry.
///
/// Scans installed plugins to build the watcher registry, then registers OS
/// file watches for:
///
///   - Every unique directory prefix derived from plugin-declared watch paths.
///   - The infrastructure directories `.orqa/` and `plugins/`.
///
/// When `.orqa/` or `plugins/` changes, calls `graph_state.reload()` to update
/// the cached artifact graph and validation context in place.
///
/// Returns a [`WatchHandle`] that keeps the watcher alive for as long as it is
/// held. Drop the handle to stop watching.
///
/// Directories that do not exist are skipped with a warning. The watcher starts
/// successfully even if no plugin watches or no infrastructure directories exist.
pub fn start_watcher(project_root: &Path, graph_state: GraphState) -> notify::Result<WatchHandle> {
    let root = project_root.to_path_buf();

    // Build the registry once here for directory collection, and again inside
    // the closure so each copy is independently owned.
    let plugin_watch_dirs = {
        let reg = build_registry(project_root);
        collect_watch_dirs(project_root, &reg)
    };

    let mut debouncer = new_debouncer(
        Duration::from_millis(DEBOUNCE_MS),
        None,
        move |result: DebounceEventResult| {
            let registry = build_registry(&root);
            handle_events(result, &root, &registry, &graph_state);
        },
    )?;

    let mut watched_any = false;

    // Register infrastructure watches.
    for dir_name in INFRASTRUCTURE_WATCH_DIRS {
        let dir = project_root.join(dir_name);
        watched_any |= watch_dir(&mut debouncer, &dir);
    }

    // Register directories derived from plugin-declared watch path patterns.
    for dir in plugin_watch_dirs {
        watched_any |= watch_dir(&mut debouncer, &dir);
    }

    if !watched_any {
        warn!(
            subsystem = "watcher",
            root = %project_root.display(),
            "[watcher] no directories could be watched"
        );
    }

    Ok(WatchHandle {
        _debouncer: debouncer,
    })
}

/// Register a single directory with the debouncer, logging the result.
///
/// Returns `true` if the watch was successfully registered, `false` otherwise.
fn watch_dir(
    debouncer: &mut Debouncer<notify::RecommendedWatcher, RecommendedCache>,
    dir: &Path,
) -> bool {
    if !dir.exists() {
        info!(
            subsystem = "watcher",
            path = %dir.display(),
            "[watcher] directory does not exist — skipping"
        );
        return false;
    }

    match debouncer.watch(dir, RecursiveMode::Recursive) {
        Ok(()) => {
            info!(
                subsystem = "watcher",
                path = %dir.display(),
                "[watcher] watching directory"
            );
            true
        }
        Err(e) => {
            warn!(
                subsystem = "watcher",
                path = %dir.display(),
                error = %e,
                "[watcher] failed to watch directory — skipping"
            );
            false
        }
    }
}

/// Collect the unique directory roots that need OS-level watches from the registry.
///
/// For each watch path pattern (relative to project root), walks up the path
/// to find the deepest existing ancestor directory. Deduplicates so each root
/// is registered only once.
fn collect_watch_dirs(project_root: &Path, registry: &[WatchRegistration]) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();

    for reg in registry {
        for pattern in &reg.watch_paths {
            // Strip any glob characters to find the static directory prefix.
            let static_prefix = static_prefix_of(pattern);
            let dir = project_root.join(&static_prefix);

            // Walk up to the closest existing ancestor.
            let mut candidate = dir.as_path();
            loop {
                if candidate.exists() {
                    let owned = candidate.to_path_buf();
                    if !dirs.contains(&owned) {
                        dirs.push(owned);
                    }
                    break;
                }
                match candidate.parent() {
                    Some(p) => candidate = p,
                    None => break,
                }
            }
        }
    }

    dirs
}

/// Extract the static (non-glob) prefix of a glob pattern.
///
/// Returns the leading path segments before any glob character appears.
/// For example: `.orqa/learning/rules/**/*.md` → `.orqa/learning/rules`.
fn static_prefix_of(pattern: &str) -> String {
    // Split on `/` and take segments up to (but not including) the first
    // segment that contains a glob character.
    let static_parts: Vec<&str> = pattern
        .split('/')
        .take_while(|seg| !seg.contains(['*', '?', '[', '{']))
        .collect();

    if static_parts.is_empty() {
        String::new()
    } else {
        static_parts.join("/")
    }
}

/// Process a batch of debounced file-system events.
///
/// For each non-ignored changed path:
///   - If it falls under `.orqa/`, calls `graph_state.reload()` to update the
///     cached artifact graph and validation context, then reloads enforcement.
///   - If it falls under `plugins/`, reloads plugin discovery and also reloads
///     the graph because plugin manifests affect the validation context.
///   - If it matches any registered watch path pattern, invokes the generator.
fn handle_events(
    result: DebounceEventResult,
    root: &Path,
    registry: &[WatchRegistration],
    graph_state: &GraphState,
) {
    match result {
        Ok(events) => {
            let mut orqa_changed = false;
            let mut plugins_changed = false;

            for event in &events {
                let relevant_paths: Vec<&PathBuf> =
                    event.paths.iter().filter(|p| !is_ignored(p)).collect();

                if relevant_paths.is_empty() {
                    continue;
                }

                debug!(
                    subsystem = "watcher",
                    changed_paths = ?relevant_paths,
                    "[watcher] file change event"
                );

                for path in &relevant_paths {
                    // Infrastructure watches.
                    if path_is_under(path, root, ".orqa") {
                        orqa_changed = true;
                    } else if path_is_under(path, root, "plugins") {
                        plugins_changed = true;
                    }

                    // Plugin-declared watches.
                    for reg in registry {
                        if matches_any_pattern(path, root, &reg.watch_paths) {
                            dispatch_action(reg, root);
                        }
                    }
                }
            }

            if orqa_changed {
                // Reload the shared graph cache so all route handlers see
                // updated state.  The reload also re-parses every enforcement
                // rule file under `.orqa/learning/rules/` and caches the
                // result — route handlers never hit disk.
                graph_state.reload(root);
            }

            if plugins_changed {
                reload_plugins(root);
                // Plugin manifests affect the validation context — also reload
                // graph (which also reloads enforcement rules, since they
                // travel together).
                graph_state.reload(root);
            }
        }
        Err(errors) => {
            for e in errors {
                warn!(
                    subsystem = "watcher",
                    error = %e,
                    "[watcher] notify error"
                );
            }
        }
    }
}

/// Dispatch the registered action for a matched watch registration.
///
/// Currently only `Regenerate` is supported; this function is the single
/// dispatch point so future actions can be added here.
fn dispatch_action(reg: &WatchRegistration, root: &Path) {
    match reg.on_change {
        WatchAction::Regenerate => invoke_generator(reg, root),
    }
}

/// Return `true` if `path` matches any of the glob `patterns` relative to `root`.
///
/// Each pattern is joined to the project root to form an absolute glob, then
/// tested against the changed path using the `glob` crate.
fn matches_any_pattern(path: &Path, root: &Path, patterns: &[String]) -> bool {
    for pattern in patterns {
        let abs_pattern = root.join(pattern);
        let pattern_str = abs_pattern.to_string_lossy();

        let glob_pattern = match glob::Pattern::new(&pattern_str) {
            Ok(p) => p,
            Err(e) => {
                warn!(
                    subsystem = "watcher",
                    pattern = %pattern_str,
                    error = %e,
                    "[watcher] invalid glob pattern — skipping"
                );
                continue;
            }
        };

        if glob_pattern.matches_path(path) {
            return true;
        }
    }
    false
}

/// Log the result of a completed generator run.
///
/// Emits an info log on success and a warn log with the exit code and captured
/// stderr on failure. Separated from `invoke_generator` to keep that function
/// within clippy's line-count limit.
fn log_generator_result(reg: &WatchRegistration, result: std::io::Result<std::process::Output>) {
    match result {
        Ok(output) => {
            if output.status.success() {
                info!(subsystem = "watcher", plugin = %reg.plugin_name,
                    engine = %reg.engine, "[watcher] generator completed successfully");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                warn!(subsystem = "watcher", plugin = %reg.plugin_name,
                    engine = %reg.engine, exit_code = ?output.status.code(),
                    stderr = %stderr, "[watcher] generator exited with error");
            }
        }
        Err(e) => {
            warn!(subsystem = "watcher", plugin = %reg.plugin_name,
                engine = %reg.engine, error = %e, "[watcher] failed to spawn generator");
        }
    }
}

/// Invoke the generator script for a matching watch registration.
///
/// Spawns the generator as a subprocess with the standard argument set defined
/// in the enforcement plugin model (section 3.1 of the design document):
///   --project-root <absolute path>
///   --output <absolute config output path>
///   --rules-dir <absolute path to .orqa/learning/rules/>
///   --filter <frontmatter filter string>   (only when filter is set)
///
/// The generator writes its output to `--output`. stderr is captured and
/// logged. stdout is ignored per the generator contract. Exit code 0 is
/// success; non-zero is logged as an error.
fn invoke_generator(reg: &WatchRegistration, root: &Path) {
    let rules_dir = root.join(".orqa/learning/rules");

    info!(subsystem = "watcher", plugin = %reg.plugin_name, engine = %reg.engine,
        generator = %reg.generator_path.display(), "[watcher] invoking generator");

    let mut cmd = Command::new(&reg.generator_path);
    cmd.arg("--project-root")
        .arg(root)
        .arg("--output")
        .arg(&reg.config_output)
        .arg("--rules-dir")
        .arg(&rules_dir);

    if let Some(filter) = &reg.filter {
        cmd.arg("--filter").arg(filter);
    }

    // On Windows, CREATE_NO_WINDOW prevents the generator subprocess from
    // opening a visible console window when it runs in the background.
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let gen_start = Instant::now();
    let result = cmd.output();
    let elapsed_ms = gen_start.elapsed().as_millis() as u64;

    info!(
        subsystem = "watcher",
        plugin = %reg.plugin_name,
        engine = %reg.engine,
        elapsed_ms,
        "[watcher] generator finished"
    );

    log_generator_result(reg, result);
}

/// Return `true` if `path` is located under `root/subdir`.
///
/// Used to classify watch events by which top-level watched directory they
/// belong to without doing a full path prefix match that could fail on
/// symlinks or non-canonical paths.
fn path_is_under(path: &Path, root: &Path, subdir: &str) -> bool {
    path.starts_with(root.join(subdir))
}

/// Reload plugin discovery after `plugins/` changes.
///
/// Calls `scan_plugins` from the engine and logs the discovered count at info
/// level. Plugin discovery is infallible (returns an empty vec on any error),
/// so this function always succeeds.
fn reload_plugins(root: &Path) {
    let plugins = scan_plugins(root);
    info!(
        subsystem = "watcher",
        plugin_count = plugins.len(),
        "[watcher] plugins reloaded ({} plugins discovered)",
        plugins.len()
    );
}

/// Return `true` if any path component of `path` is an ignored directory.
///
/// Checks every component of the path string representation against the
/// `IGNORED_COMPONENTS` list. This catches paths like
/// `/project/node_modules/foo/bar.md` regardless of nesting depth.
fn is_ignored(path: &Path) -> bool {
    path.components().any(|comp| {
        let s = comp.as_os_str().to_string_lossy();
        IGNORED_COMPONENTS
            .iter()
            .any(|ignored| s.as_ref() == *ignored)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn node_modules_is_ignored() {
        let p = PathBuf::from("/project/node_modules/lodash/index.js");
        assert!(is_ignored(&p));
    }

    #[test]
    fn git_dir_is_ignored() {
        let p = PathBuf::from("/project/.git/COMMIT_EDITMSG");
        assert!(is_ignored(&p));
    }

    #[test]
    fn target_dir_is_ignored() {
        let p = PathBuf::from("/project/target/debug/build/orqa-daemon.d");
        assert!(is_ignored(&p));
    }

    #[test]
    fn state_dir_is_ignored() {
        let p = PathBuf::from("/project/.state/daemon.pid");
        assert!(is_ignored(&p));
    }

    #[test]
    fn orqa_artifact_is_not_ignored() {
        let p = PathBuf::from("/project/.orqa/planning/epics/EPIC-001.md");
        assert!(!is_ignored(&p));
    }

    #[test]
    fn plugin_file_is_not_ignored() {
        let p = PathBuf::from("/project/plugins/agile/orqa-plugin.json");
        assert!(!is_ignored(&p));
    }

    #[test]
    fn path_is_under_detects_orqa() {
        let root = PathBuf::from("/project");
        let path = PathBuf::from("/project/.orqa/learning/rules/rule-001.md");
        assert!(path_is_under(&path, &root, ".orqa"));
        assert!(!path_is_under(&path, &root, "plugins"));
    }

    #[test]
    fn path_is_under_detects_plugins() {
        let root = PathBuf::from("/project");
        let path = PathBuf::from("/project/plugins/agile/orqa-plugin.json");
        assert!(path_is_under(&path, &root, "plugins"));
        assert!(!path_is_under(&path, &root, ".orqa"));
    }

    #[test]
    fn reload_plugins_does_not_crash_on_missing_project() {
        // scan_plugins is infallible — returns empty vec for nonexistent root.
        reload_plugins(Path::new("/nonexistent/project/root"));
    }

    #[test]
    fn static_prefix_extracts_up_to_first_glob() {
        // Glob patterns should yield the deepest static directory prefix.
        assert_eq!(
            static_prefix_of(".orqa/learning/rules/**/*.md"),
            ".orqa/learning/rules"
        );
        assert_eq!(static_prefix_of("plugins/**/orqa-plugin.json"), "plugins");
        assert_eq!(static_prefix_of(".orqa/**/*.md"), ".orqa");
    }

    #[test]
    fn static_prefix_handles_no_glob() {
        // A plain path with no glob characters returns the full path.
        assert_eq!(
            static_prefix_of(".orqa/learning/rules"),
            ".orqa/learning/rules"
        );
    }

    #[test]
    fn matches_any_pattern_matches_correctly() {
        // A path that matches the pattern should return true.
        let root = PathBuf::from("/project");
        let path = PathBuf::from("/project/.orqa/learning/rules/ts/rule-001.md");
        let patterns = vec![".orqa/learning/rules/**/*.md".to_owned()];
        assert!(matches_any_pattern(&path, &root, &patterns));
    }

    #[test]
    fn matches_any_pattern_rejects_non_matching() {
        // A path that does not match should return false.
        let root = PathBuf::from("/project");
        let path = PathBuf::from("/project/.orqa/configs/eslint.config.js");
        let patterns = vec![".orqa/learning/rules/**/*.md".to_owned()];
        assert!(!matches_any_pattern(&path, &root, &patterns));
    }

    #[test]
    fn build_registry_returns_empty_for_nonexistent_root() {
        // No plugins exist for a nonexistent project root — registry must be empty.
        let registry = build_registry(Path::new("/nonexistent/project/root"));
        assert!(registry.is_empty());
    }

    #[test]
    fn collect_watch_dirs_empty_registry() {
        // An empty registry should yield no extra watch directories.
        let root = PathBuf::from("/project");
        let dirs = collect_watch_dirs(&root, &[]);
        assert!(dirs.is_empty());
    }
}
