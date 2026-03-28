/**
 * Connector file watcher — watches plugin and governance files for changes
 * and triggers regeneration of the Claude Code Plugin via generator.ts.
 *
 * Watches:
 *   - .orqa/workflows/*.resolved.yaml — workflow composition output
 *   - .orqa/learning/rules/*.md       — active rule changes
 *   - plugins/*\/orqa-plugin.json     — plugin manifest changes
 *   - .orqa/schema.composed.json      — composed schema changes
 *
 * Regeneration is debounced to prevent excessive triggering during rapid
 * file system events (e.g. editor save-then-format sequences).
 *
 * Graceful cleanup: the returned cleanup function removes all watchers on
 * process exit. Callers should invoke it in SIGTERM/SIGINT handlers.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import { generatePlugin } from "./generator.js";

/** Debounce delay in milliseconds. */
const DEBOUNCE_MS = 500;

/** Paths to watch relative to project root. */
const WATCH_PATTERNS = [
  ".orqa/workflows",
  ".orqa/learning/rules",
  ".orqa/schema.composed.json",
];

/** Glob pattern (directory) for plugin manifests. */
const PLUGINS_DIR = "plugins";

/** File name to match for plugin manifests inside plugin directories. */
const PLUGIN_MANIFEST_NAME = "orqa-plugin.json";

// ---------------------------------------------------------------------------
// Debouncer
// ---------------------------------------------------------------------------

/**
 * Create a debounced wrapper around a callback.
 *
 * Any call within `delayMs` of the previous call resets the timer.
 * Only one execution fires per burst of rapid calls.
 */
function debounce(fn: () => void, delayMs: number): () => void {
  let timer: ReturnType<typeof setTimeout> | null = null;
  return () => {
    if (timer !== null) clearTimeout(timer);
    timer = setTimeout(() => {
      timer = null;
      fn();
    }, delayMs);
  };
}

// ---------------------------------------------------------------------------
// Watcher setup
// ---------------------------------------------------------------------------

/**
 * Start a file watcher on a path that may not exist yet.
 *
 * If the path does not exist, logs a warning and returns null.
 * The caller must handle the null case gracefully.
 */
function startWatcher(
  watchPath: string,
  onChange: () => void,
  label: string,
): fs.FSWatcher | null {
  if (!fs.existsSync(watchPath)) {
    // Not an error — many watch targets may not exist in fresh projects.
    return null;
  }

  try {
    const watcher = fs.watch(watchPath, { recursive: true }, (_event, filename) => {
      // Filter to relevant file types.
      if (
        filename === null ||
        (!filename.endsWith(".yaml") &&
          !filename.endsWith(".yml") &&
          !filename.endsWith(".md") &&
          !filename.endsWith(".json"))
      ) {
        return;
      }
      console.log(`[watcher] ${label}: changed — ${filename}`);
      onChange();
    });
    return watcher;
  } catch {
    // fs.watch can fail on some platforms for certain paths.
    console.warn(`[watcher] could not watch ${label} — ${watchPath}`);
    return null;
  }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Watch for changes to plugin manifests, rules, workflow resolutions,
 * and schema composition — and regenerate the Claude Code Plugin artifacts
 * when any change is detected.
 *
 * Debounces rapid change bursts to a single regeneration after DEBOUNCE_MS.
 * Logs each detected change and each regeneration.
 *
 * @param projectRoot - The project root directory.
 * @returns A cleanup function that stops all watchers. Call on process exit.
 */
export function watchAndRegenerate(projectRoot: string): () => void {
  const watchers: fs.FSWatcher[] = [];

  const onChanged = debounce(() => {
    console.log("[watcher] regenerating Claude Code Plugin artifacts...");
    try {
      const result = generatePlugin(projectRoot);
      const written = [result.claudeMd, result.hooksJson, result.mcpJson, result.lspJson, ...result.agents];
      console.log(
        `[watcher] regeneration complete — ${written.length} file(s) written`,
      );
      if (result.errors.length > 0) {
        for (const w of result.errors) {
          console.warn(`[watcher] warning: ${w}`);
        }
      }
    } catch (err) {
      console.error(
        `[watcher] regeneration failed: ${err instanceof Error ? err.message : String(err)}`,
      );
    }
  }, DEBOUNCE_MS);

  // Watch .orqa/workflows/, .orqa/learning/rules/, .orqa/schema.composed.json.
  for (const pattern of WATCH_PATTERNS) {
    const watchPath = path.join(projectRoot, pattern);
    const watcher = startWatcher(watchPath, onChanged, pattern);
    if (watcher !== null) watchers.push(watcher);
  }

  // Watch plugins/*/orqa-plugin.json by watching the plugins/ directory.
  // fs.watch with recursive:true on the directory will catch manifest changes.
  const pluginsDir = path.join(projectRoot, PLUGINS_DIR);
  if (fs.existsSync(pluginsDir)) {
    try {
      const watcher = fs.watch(
        pluginsDir,
        { recursive: true },
        (_event, filename) => {
          if (filename !== null && filename.endsWith(PLUGIN_MANIFEST_NAME)) {
            console.log(`[watcher] plugins: manifest changed — ${filename}`);
            onChanged();
          }
        },
      );
      watchers.push(watcher);
    } catch {
      console.warn(`[watcher] could not watch plugins directory — ${pluginsDir}`);
    }
  }

  console.log(`[watcher] watching ${watchers.length} path(s) in ${projectRoot}`);

  // Cleanup function — removes all watchers.
  return () => {
    for (const watcher of watchers) {
      watcher.close();
    }
    console.log("[watcher] all watchers closed");
  };
}
