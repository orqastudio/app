/**
 * Plugin Store — manages plugin lifecycle operations via Tauri IPC.
 *
 * Components must not call invoke() directly (RULE-006). All plugin-related
 * Tauri commands are routed through this store so components can bind to
 * reactive state and call plain async methods instead.
 */

import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import type { PluginManifest, CliToolRunResult, CliToolRunStatus } from "@orqastudio/types";

/** A lightweight plugin entry as returned by plugin_list_installed and plugin_registry_list. */
export interface PluginEntry {
	name: string;
	displayName?: string;
	display_name?: string;
	description?: string;
	version?: string;
	path?: string;
	source?: string;
	repo?: string;
	category?: string;
	icon?: string;
	capabilities?: string[];
}

/** Result from plugin_registry_list. */
interface RegistryListResult {
	plugins: PluginEntry[];
}

/** Reactive store for plugin lifecycle: install, uninstall, registry listing, and CLI tool runs. */
export class PluginStore {
	/** List of currently installed plugins. */
	installed = $state<PluginEntry[]>([]);
	/** True while installed plugins are being loaded. */
	loadingInstalled = $state(false);
	/** True while the plugin registry is being fetched. */
	loadingRegistry = $state(false);
	/** Current run status for all registered CLI tools. */
	cliToolStatuses = $state<CliToolRunStatus[]>([]);
	/** Last error message, or null if none. */
	error = $state<string | null>(null);

	/**
	 * Load the list of currently installed plugins from the backend.
	 * Results are stored in `installed`.
	 */
	async loadInstalled(): Promise<void> {
		this.loadingInstalled = true;
		this.error = null;
		try {
			this.installed = await invoke<PluginEntry[]>("plugin_list_installed");
		} catch (err) {
			this.error = extractErrorMessage(err);
			this.installed = [];
		} finally {
			this.loadingInstalled = false;
		}
	}

	/**
	 * Fetch plugin listing from the official or community registry.
	 * Returns the plugin list rather than storing it, because callers may
	 * need to merge with other sources before displaying.
	 * @param source - Which registry to query: "official" or "community".
	 * @returns Array of plugin entries from the registry.
	 */
	async listRegistry(source: "official" | "community"): Promise<PluginEntry[]> {
		this.loadingRegistry = true;
		this.error = null;
		try {
			const result = await invoke<RegistryListResult>("plugin_registry_list", { source });
			return result.plugins;
		} catch (err) {
			this.error = extractErrorMessage(err);
			return [];
		} finally {
			this.loadingRegistry = false;
		}
	}

	/**
	 * Install a plugin from a GitHub repository.
	 * Caller should call loadInstalled() afterwards to refresh the list.
	 * @param repo - GitHub repository slug (e.g. "owner/repo").
	 * @param version - Optional version tag or commit; defaults to latest.
	 */
	async installFromGitHub(repo: string, version?: string | null): Promise<void> {
		this.error = null;
		try {
			await invoke("plugin_install_github", { repo, version: version ?? null });
		} catch (err) {
			this.error = extractErrorMessage(err);
			throw err;
		}
	}

	/**
	 * Install a plugin from a local filesystem path.
	 * Caller should call loadInstalled() afterwards to refresh the list.
	 * @param path - Absolute path to the local plugin directory.
	 */
	async installFromLocal(path: string): Promise<void> {
		this.error = null;
		try {
			await invoke("plugin_install_local", { path });
		} catch (err) {
			this.error = extractErrorMessage(err);
			throw err;
		}
	}

	/**
	 * Fetch the full manifest for an installed plugin by name.
	 * Returns null and sets error if not found or the call fails.
	 * @param name - Plugin name to look up.
	 * @returns The plugin manifest, or null on failure.
	 */
	async getManifest(name: string): Promise<PluginManifest | null> {
		this.error = null;
		try {
			return await invoke<PluginManifest>("plugin_get_manifest", { name });
		} catch (err) {
			this.error = extractErrorMessage(err);
			return null;
		}
	}

	/**
	 * Uninstall a plugin by name.
	 * Caller should call loadInstalled() afterwards to refresh the list.
	 * @param name - Plugin name to uninstall.
	 */
	async uninstall(name: string): Promise<void> {
		this.error = null;
		try {
			await invoke("plugin_uninstall", { name });
		} catch (err) {
			this.error = extractErrorMessage(err);
			throw err;
		}
	}

	/**
	 * Load the current run status for all registered CLI tools.
	 * Results are stored in `cliToolStatuses`.
	 */
	async loadCliToolStatuses(): Promise<void> {
		this.error = null;
		try {
			this.cliToolStatuses = await invoke<CliToolRunStatus[]>("cli_tool_status");
		} catch (err) {
			this.error = extractErrorMessage(err);
			this.cliToolStatuses = [];
		}
	}

	/**
	 * Run a specific CLI tool by plugin name and tool key.
	 * Returns the run result. Reloads tool statuses on success.
	 * @param pluginName - Name of the plugin that owns the tool.
	 * @param toolKey - Key identifying the specific tool to run.
	 * @returns The tool run result.
	 */
	async runCliTool(pluginName: string, toolKey: string): Promise<CliToolRunResult> {
		this.error = null;
		try {
			const result = await invoke<CliToolRunResult>("run_cli_tool", {
				pluginName,
				toolKey,
			});
			await this.loadCliToolStatuses();
			return result;
		} catch (err) {
			this.error = extractErrorMessage(err);
			throw err;
		}
	}
}
