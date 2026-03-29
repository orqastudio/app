/**
 * Plugin service — Tauri IPC wrappers for plugin-related backend commands.
 *
 * Components must not call invoke() directly (RULE-006: component purity).
 * All Tauri IPC calls related to plugins live here so components remain
 * pure presentational units that receive data via props or stores.
 */

import { invoke } from "@tauri-apps/api/core";

/**
 * Retrieve the filesystem path of an installed plugin by name.
 *
 * Calls the Tauri `plugin_get_path` command and returns the absolute path
 * string. Throws if the plugin is not installed or the backend returns an error.
 * @param name - The plugin name to look up.
 * @returns The absolute filesystem path of the installed plugin.
 */
export async function getPluginPath(name: string): Promise<string> {
	return invoke<string>("plugin_get_path", { name });
}
