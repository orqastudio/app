<script lang="ts">
	import { Icon, HStack, Stack, Text } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardContent, CardAction } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Input } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { getStores, logger } from "@orqastudio/sdk";
	import type { PluginEntry } from "@orqastudio/sdk";

	const log = logger("plugin-browser");
	import type { RegistrationConflict } from "@orqastudio/sdk";
	import type { PluginManifest } from "@orqastudio/types";
	import ConflictResolutionDialog from "./ConflictResolutionDialog.svelte";

	const { pluginRegistry, pluginStore } = getStores();

	// -----------------------------------------------------------------------
	// Types
	// -----------------------------------------------------------------------

	type Tab = "installed" | "official" | "community" | "groups";
	type DetailView = { type: "installed" | "registry"; plugin: PluginEntry } | null;

	/** A plugin bundle — a named group of plugins that install together. */
	interface PluginBundle {
		key: string;
		label: string;
		description: string;
		icon: string;
		plugins: PluginEntry[];
	}

	interface PluginManifestData {
		name: string;
		version: string;
		display_name?: string;
		description?: string;
		provides: {
			schemas: Array<{ key: string; label: string; icon: string }>;
			views: Array<{ key: string; label: string; icon: string }>;
			widgets: Array<{ key: string; label: string; icon: string }>;
			relationships: Array<{ key: string; inverse: string; label: string; description: string }>;
			cli_tools: Array<{ key: string; label: string }>;
			hooks: Array<{ key: string; event: string }>;
		};
	}

	// -----------------------------------------------------------------------
	// State
	// -----------------------------------------------------------------------

	let activeTab = $state<Tab>("installed");
	let official = $state<PluginEntry[]>([]);
	let community = $state<PluginEntry[]>([]);
	let error = $state<string | null>(null);
	let manualSource = $state("");
	let installing = $state<string | null>(null);
	let installingBundle = $state<string | null>(null);
	let detailView = $state<DetailView>(null);
	let detailManifest = $state<PluginManifestData | null>(null);
	let detailLoading = $state(false);

	// Conflict resolution
	let conflictDialog = $state<{
		conflicts: RegistrationConflict[];
		existingManifest: PluginManifest;
		newManifest: PluginManifest;
		pendingPlugin: PluginEntry;
	} | null>(null);

	// -----------------------------------------------------------------------
	// Data Loading
	// -----------------------------------------------------------------------

	$effect(() => {
		void loadInstalled();
	});

	/** Core plugin names to filter from all views. These are infrastructure, not user-facing. */
	const CORE_PLUGIN_NAMES = new Set(["@orqastudio/plugin-core-framework", "core", "@orqastudio/core"]);

	/**
	 * Returns true if the plugin is the core framework plugin, which is hidden from the browser.
	 * @param plugin - The plugin entry to check.
	 * @returns Whether the plugin is a core infrastructure plugin.
	 */
	function isCorePlugin(plugin: PluginEntry): boolean {
		return CORE_PLUGIN_NAMES.has(plugin.name);
	}

	/** Loads the installed plugin list and logs any errors. */
	async function loadInstalled() {
		await pluginStore.loadInstalled();
		if (pluginStore.error) {
			log.error("Failed to load installed plugins", { err: pluginStore.error });
		}
	}

	/** Derive the visible installed list (core infrastructure hidden from browser). */
	const installed = $derived(pluginStore.installed.filter((p) => !isCorePlugin(p)));

	/**
	 * Loads plugins from the specified registry source, filtering out core plugins.
	 * @param source - The registry source to load from: "official" or "community".
	 */
	async function loadRegistry(source: "official" | "community") {
		error = null;
		const plugins = await pluginStore.listRegistry(source);
		const filtered = plugins.filter((p) => !isCorePlugin(p));
		if (source === "official") official = filtered;
		else community = filtered;
		if (pluginStore.error) {
			error = pluginStore.error;
		}
	}

	/**
	 * Derive plugin bundles from the official registry.
	 * Bundles group related plugins by taxonomy (methodology + related workflows).
	 * A bundle whose key matches a category has all plugins in that category.
	 */
	const bundles = $derived.by((): PluginBundle[] => {
		const allRegistryPlugins = [...official, ...community];
		if (allRegistryPlugins.length === 0) return [];

		// Group by category, creating one bundle per category.
		const byCategory: Record<string, PluginEntry[]> = {};
		for (const plugin of allRegistryPlugins) {
			const cat = plugin.category ?? "other";
			if (!byCategory[cat]) byCategory[cat] = [];
			byCategory[cat].push(plugin);
		}

		// Only surface bundles with more than one plugin (single-plugin categories
		// are better browsed in the individual tabs).
		const result: PluginBundle[] = [];
		for (const [cat, plugins] of Object.entries(byCategory)) {
			if (plugins.length < 2) continue;
			result.push({
				key: cat,
				label: cat.replace(/-/g, " ").replace(/\b\w/g, (c) => c.toUpperCase()),
				description: `Install all ${plugins.length} plugins in this category together.`,
				icon: categoryIcon(cat),
				plugins,
			});
		}
		return result;
	});

	/**
	 * Maps a plugin category key to a Lucide icon name for display.
	 * @param category - The plugin category key.
	 * @returns A Lucide icon name string.
	 */
	function categoryIcon(category: string): string {
		const icons: Record<string, string> = {
			methodology: "compass",
			workflow: "git-branch",
			knowledge: "brain",
			infrastructure: "server",
			connector: "plug",
			sidecar: "bot",
			tooling: "wrench",
			"coding-standards": "code-2",
			enforcement: "shield",
		};
		return icons[category] ?? "package";
	}

	/**
	 * Installs all plugins in a bundle sequentially, skipping already-installed ones.
	 * @param bundle - The plugin bundle to install.
	 */
	async function installBundle(bundle: PluginBundle) {
		installingBundle = bundle.key;
		error = null;
		try {
			for (const plugin of bundle.plugins) {
				if (!plugin.repo || isInstalled(plugin.name)) continue;
				await pluginStore.installFromGitHub(plugin.repo);
			}
			await loadInstalled();
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			installingBundle = null;
		}
	}

	/**
	 * Returns true if every plugin in the bundle is already installed.
	 * @param bundle - The plugin bundle to check.
	 * @returns Whether all plugins in the bundle are installed.
	 */
	function isBundleInstalled(bundle: PluginBundle): boolean {
		return bundle.plugins.every((p) => isInstalled(p.name));
	}

	/**
	 * Switches the active tab and lazily loads registry data for tabs that require it.
	 * @param tab - The tab to activate.
	 */
	async function handleTabChange(tab: Tab) {
		activeTab = tab;
		detailView = null;
		if (tab === "official" && official.length === 0) await loadRegistry("official");
		if (tab === "community" && community.length === 0) await loadRegistry("community");
		if (tab === "groups" && official.length === 0) await loadRegistry("official");
	}

	// -----------------------------------------------------------------------
	// Install / Uninstall
	// -----------------------------------------------------------------------

	/**
	 * Downloads and registers a plugin from the registry, triggering conflict resolution if needed.
	 * @param plugin - The registry plugin entry to install.
	 */
	async function installFromRegistry(plugin: PluginEntry) {
		if (!plugin.repo) return;
		installing = plugin.name;
		error = null;
		try {
			await pluginStore.installFromGitHub(plugin.repo);

			// Read the installed manifest and check for conflicts before registering
			const manifest = await pluginStore.getManifest(plugin.name);
			if (!manifest) {
				error = pluginStore.error ?? "Failed to read plugin manifest";
				installing = null;
				return;
			}
			const conflicts = pluginRegistry.checkConflicts(manifest);

			if (conflicts.length > 0) {
				// Find the existing plugin that owns the conflicting key
				const existingName = conflicts[0].existingPlugin;
				const existingPlugin = pluginRegistry.getPlugin(existingName);
				const existingManifest = existingPlugin?.manifest ?? manifest;

				conflictDialog = {
					conflicts,
					existingManifest,
					newManifest: manifest,
					pendingPlugin: plugin,
				};
				installing = null;
				return;
			}

			// No conflicts — register directly
			pluginRegistry.register(manifest, {});
			await loadInstalled();
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			installing = null;
		}
	}

	/**
	 * Applies the user's conflict resolutions and retries plugin registration.
	 * @param resolutions - A map of conflict key to the chosen alias configuration.
	 */
	async function handleConflictResolution(
		resolutions: Record<string, { plugin: string; alias: string; label?: string }>,
	) {
		if (!conflictDialog) return;
		error = null;

		try {
			// Apply aliases
			for (const [key, resolution] of Object.entries(resolutions)) {
				const isSchema = conflictDialog.conflicts.some(
					(c) => c.key === key && c.type === "schema",
				);
				pluginRegistry.setAlias(
					resolution.plugin,
					isSchema ? "schema" : "relationship",
					key,
					resolution.alias,
					resolution.label,
				);
			}

			// Retry registration
			pluginRegistry.register(conflictDialog.newManifest, {});
			await loadInstalled();
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			conflictDialog = null;
		}
	}

	/** Cancels conflict resolution and uninstalls the downloaded plugin that failed to register. */
	function handleConflictCancel() {
		if (conflictDialog) {
			// Uninstall the plugin that was downloaded but couldn't register
			void uninstallPlugin(conflictDialog.pendingPlugin.name);
		}
		conflictDialog = null;
	}

	/** Installs a plugin from the manually entered GitHub repo path or local filesystem path. */
	async function installManual() {
		if (!manualSource.trim()) return;
		installing = "manual";
		error = null;
		try {
			const source = manualSource.trim();
			if (source.includes("/") && !source.includes("\\") && !source.includes(":")) {
				// GitHub repo format: owner/repo or owner/repo@version
				const [repo, version] = source.split("@");
				await pluginStore.installFromGitHub(repo, version ?? null);
			} else {
				await pluginStore.installFromLocal(source);
			}
			manualSource = "";
			await loadInstalled();
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			installing = null;
		}
	}

	/**
	 * Uninstalls a plugin by name and refreshes the installed list.
	 * @param name - The plugin name to uninstall.
	 */
	async function uninstallPlugin(name: string) {
		error = null;
		try {
			await pluginStore.uninstall(name);
			await loadInstalled();
			if (detailView?.plugin.name === name) detailView = null;
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		}
	}

	// -----------------------------------------------------------------------
	// Detail View
	// -----------------------------------------------------------------------

	/**
	 * Opens the detail view for a plugin, loading its manifest if it is installed.
	 * @param plugin - The plugin entry to show.
	 * @param type - Whether the plugin is from the "installed" list or a "registry" source.
	 */
	async function showDetail(plugin: PluginEntry, type: "installed" | "registry") {
		detailView = { type, plugin };
		detailManifest = null;

		if (type === "installed") {
			detailLoading = true;
			try {
				detailManifest = await pluginStore.getManifest(plugin.name) as PluginManifestData | null;
			} catch (err) {
				log.error("Failed to load plugin manifest for detail view", { pluginName: plugin.name, err });
				detailManifest = null;
			} finally {
				detailLoading = false;
			}
		}
	}

	/** Closes the detail panel and clears the detail state. */
	function closeDetail() {
		detailView = null;
		detailManifest = null;
	}

	// -----------------------------------------------------------------------
	// Helpers
	// -----------------------------------------------------------------------

	/**
	 * Returns the best available display name for a plugin entry.
	 * @param plugin - The plugin entry to get a display name for.
	 * @returns The display name, falling back to the plugin's name field.
	 */
	function displayName(plugin: PluginEntry): string {
		return plugin.displayName ?? plugin.display_name ?? plugin.name;
	}

	/**
	 * Returns true if a plugin with the given name is in the installed list.
	 * @param name - The plugin name to check.
	 * @returns Whether the plugin is currently installed.
	 */
	function isInstalled(name: string): boolean {
		return installed.some((p) => p.name === name);
	}
</script>

<div class="space-y-4">
	<!-- Header -->
	<CardRoot>
		<CardHeader>
			<CardTitle>
				<HStack gap={2}>
					<Icon name="puzzle" size="md" />
					Plugins
				</HStack>
			</CardTitle>
			<CardAction>
				<Badge variant="outline" size="xs">
					{installed.length} installed
				</Badge>
			</CardAction>
		</CardHeader>
	</CardRoot>

	{#if detailView}
		<!-- Detail View -->
		<div class="space-y-3">
			<Button
				variant="ghost"
				size="sm"
				onclick={closeDetail}
			>
				<Icon name="arrow-left" size="sm" />
				Back to {activeTab}
			</Button>

			<CardRoot>
				<CardHeader>
					<CardTitle>
						<HStack gap={2}>
							<Icon name={detailView.plugin.icon ?? "puzzle"} size="md" />
							{displayName(detailView.plugin)}
						</HStack>
					</CardTitle>
					<CardAction>
						{#if detailView.type === "installed"}
							<Button
								variant="ghost"
								size="sm"
								onclick={() => detailView && uninstallPlugin(detailView.plugin.name)}
							>
								Uninstall
							</Button>
						{:else if !isInstalled(detailView.plugin.name)}
							<Button
								variant="default"
								size="sm"
								disabled={installing !== null}
								onclick={() => detailView && installFromRegistry(detailView.plugin)}
							>
								{#if installing === detailView.plugin.name}
									<LoadingSpinner size="sm" />
									Installing...
								{:else}
									Install
								{/if}
							</Button>
						{:else}
							<Badge variant="outline" size="xs">Installed</Badge>
						{/if}
					</CardAction>
				</CardHeader>
				<CardContent>
					<Text variant="caption" tone="muted">{detailView.plugin.description ?? "No description"}</Text>
					<div class="mt-2 flex items-center gap-2">
						{#if detailView.plugin.version}
							<Text variant="caption" tone="muted">v{detailView.plugin.version}</Text>
						{/if}
						{#if detailView.plugin.repo}
							<Text variant="caption" tone="muted">{detailView.plugin.repo}</Text>
						{/if}
						{#if detailView.plugin.source}
							<Text variant="caption" tone="muted">{detailView.plugin.source}</Text>
						{/if}
					</div>
					{#if detailView.plugin.capabilities?.length}
						<div class="mt-2 flex flex-wrap gap-1">
							{#each detailView.plugin.capabilities as cap (cap)}
								<Badge variant="outline" size="xs">{cap}</Badge>
							{/each}
						</div>
					{/if}
				</CardContent>
			</CardRoot>

			<!-- Manifest details (installed plugins only) -->
			{#if detailLoading}
				<div class="flex items-center justify-center py-6">
					<LoadingSpinner size="md" />
				</div>
			{:else if detailManifest}
				{#if detailManifest.provides.schemas.length > 0}
					<CardRoot gap={1}>
						<CardHeader>
							<CardTitle>Artifact Schemas ({detailManifest.provides.schemas.length})</CardTitle>
						</CardHeader>
						<CardContent>
							<Stack gap={1}>
								{#each detailManifest.provides.schemas as schema (schema.key)}
									<HStack gap={2}>
										<Icon name={schema.icon} size="sm" />
										<span class="text-xs font-medium">{schema.label}</span>
										<Text variant="caption" tone="muted">({schema.key})</Text>
									</HStack>
								{/each}
							</Stack>
						</CardContent>
					</CardRoot>
				{/if}

				{#if detailManifest.provides.relationships.length > 0}
					<CardRoot gap={1}>
						<CardHeader>
							<CardTitle>Relationships ({detailManifest.provides.relationships.length})</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="space-y-1">
								{#each detailManifest.provides.relationships as rel (rel.key)}
									<div class="text-xs">
										<span class="font-medium">{rel.label}</span>
										<span class="text-muted-foreground"> / {rel.inverse}</span>
										<p class="text-[10px] text-muted-foreground">{rel.description}</p>
									</div>
								{/each}
							</div>
						</CardContent>
					</CardRoot>
				{/if}

				{#if detailManifest.provides.views.length > 0}
					<CardRoot gap={1}>
						<CardHeader>
							<CardTitle>Views ({detailManifest.provides.views.length})</CardTitle>
						</CardHeader>
						<CardContent>
							<Stack gap={1}>
								{#each detailManifest.provides.views as view (view.key)}
									<HStack gap={2}>
										<Icon name={view.icon} size="sm" />
										<Text variant="caption">{view.label}</Text>
									</HStack>
								{/each}
							</Stack>
						</CardContent>
					</CardRoot>
				{/if}

				{#if detailManifest.provides.widgets.length > 0}
					<CardRoot gap={1}>
						<CardHeader>
							<CardTitle>Widgets ({detailManifest.provides.widgets.length})</CardTitle>
						</CardHeader>
						<CardContent>
							<Stack gap={1}>
								{#each detailManifest.provides.widgets as widget (widget.key)}
									<HStack gap={2}>
										<Icon name={widget.icon} size="sm" />
										<Text variant="caption">{widget.label}</Text>
									</HStack>
								{/each}
							</Stack>
						</CardContent>
					</CardRoot>
				{/if}

				{#if detailManifest.provides.cli_tools.length > 0 || detailManifest.provides.hooks.length > 0}
					<CardRoot gap={1}>
						<CardHeader>
							<CardTitle>Backend Capabilities</CardTitle>
						</CardHeader>
						<CardContent>
							<Stack gap={1}>
								{#each detailManifest.provides.cli_tools as tool (tool.key)}
									<HStack gap={2}>
										<Icon name="terminal" size="sm" />
										<Text variant="caption">{tool.label}</Text>
									</HStack>
								{/each}
								{#each detailManifest.provides.hooks as hook (hook.event)}
									<HStack gap={2}>
										<Icon name="webhook" size="sm" />
										<Text variant="caption">{hook.key}</Text>
										<Text variant="caption" tone="muted">({hook.event})</Text>
									</HStack>
								{/each}
							</Stack>
						</CardContent>
					</CardRoot>
				{/if}
			{/if}
		</div>
	{:else}
		<!-- Tab bar -->
		<div class="flex items-center gap-1 rounded-md border border-border p-1">
			{#each (["installed", "official", "community", "groups"] as Tab[]) as tab (tab)}
				<Button
					variant={activeTab === tab ? "default" : "ghost"}
					size="sm"
					onclick={() => handleTabChange(tab)}
				>
					{tab.charAt(0).toUpperCase() + tab.slice(1)}
				</Button>
			{/each}
		</div>

		<!-- Installed tab -->
		{#if activeTab === "installed"}
			<div class="space-y-2">
				{#each installed as plugin (plugin.name)}
					<CardRoot gap={2} interactive onclick={() => showDetail(plugin, "installed")}>
						<CardContent>
							<HStack justify="between">
								<div>
									<p class="text-xs font-medium">{displayName(plugin)}</p>
									<p class="text-[10px] text-muted-foreground">
										v{plugin.version ?? "?"} — {plugin.source ?? "local"}
									</p>
									{#if plugin.description}
										<p class="mt-1 text-[10px] text-muted-foreground line-clamp-1">{plugin.description}</p>
									{/if}
								</div>
								<HStack gap={2}>
									<Icon name="chevron-right" size="sm" />
								</HStack>
							</HStack>
						</CardContent>
					</CardRoot>
				{:else}
					<p class="py-4 text-center text-xs text-muted-foreground">
						No plugins installed yet. Browse the Official tab or use Manual Install below.
					</p>
				{/each}
			</div>

		<!-- Official tab -->
		{:else if activeTab === "official"}
			{#if pluginStore.loadingRegistry}
				<div class="flex items-center justify-center py-8">
					<LoadingSpinner size="md" />
				</div>
			{:else}
				<div class="space-y-2">
					{#each official as plugin (plugin.name)}
						<CardRoot gap={2} interactive onclick={() => showDetail(plugin, "registry")}>
							<CardContent>
								<HStack justify="between">
									<div>
										<HStack gap={2}>
											<Icon name={plugin.icon ?? "puzzle"} size="sm" />
											<p class="text-xs font-medium">{displayName(plugin)}</p>
											{#if isInstalled(plugin.name)}
												<Badge variant="outline" size="xs">Installed</Badge>
											{/if}
										</HStack>
										<p class="mt-1 text-[10px] text-muted-foreground line-clamp-1">{plugin.description}</p>
										{#if plugin.capabilities?.length}
											<div class="mt-1 flex flex-wrap gap-1">
												{#each plugin.capabilities as cap (cap)}
													<Badge variant="outline" size="xs">{cap}</Badge>
												{/each}
											</div>
										{/if}
									</div>
									<div class="flex shrink-0 items-center gap-2">
										{#if !isInstalled(plugin.name)}
											<Button
												variant="default"
												size="sm"
												disabled={installing !== null}
												onclick={(e: MouseEvent) => { e.stopPropagation(); installFromRegistry(plugin); }}
											>
												{#if installing === plugin.name}
													<LoadingSpinner size="sm" />
												{:else}
													Install
												{/if}
											</Button>
										{/if}
										<Icon name="chevron-right" size="sm" />
									</div>
								</HStack>
							</CardContent>
						</CardRoot>
					{:else}
						<p class="py-4 text-center text-xs text-muted-foreground">
							No official plugins available yet.
						</p>
					{/each}
				</div>
			{/if}

		<!-- Community tab -->
		{:else if activeTab === "community"}
			{#if pluginStore.loadingRegistry}
				<div class="flex items-center justify-center py-8">
					<LoadingSpinner size="md" />
				</div>
			{:else}
				<div class="space-y-2">
					{#each community as plugin (plugin.name)}
						<CardRoot gap={2} interactive onclick={() => showDetail(plugin, "registry")}>
							<CardContent>
								<HStack justify="between">
									<div>
										<HStack gap={2}>
											<Icon name={plugin.icon ?? "puzzle"} size="sm" />
											<p class="text-xs font-medium">{displayName(plugin)}</p>
											<Badge variant="secondary" size="xs">Community</Badge>
											{#if isInstalled(plugin.name)}
												<Badge variant="outline" size="xs">Installed</Badge>
											{/if}
										</HStack>
										<p class="mt-1 text-[10px] text-muted-foreground line-clamp-1">{plugin.description}</p>
									</div>
									<div class="flex shrink-0 items-center gap-2">
										{#if !isInstalled(plugin.name)}
											<Button
												variant="outline"
												size="sm"
												disabled={installing !== null}
												onclick={(e: MouseEvent) => { e.stopPropagation(); installFromRegistry(plugin); }}
											>
												{#if installing === plugin.name}
													<LoadingSpinner size="sm" />
												{:else}
													Install
												{/if}
											</Button>
										{/if}
										<Icon name="chevron-right" size="sm" />
									</div>
								</HStack>
							</CardContent>
						</CardRoot>
					{:else}
						<p class="py-4 text-center text-xs text-muted-foreground">
							No community plugins available yet.
						</p>
					{/each}
				</div>
			{/if}
		<!-- Groups tab — plugin bundles derived from registry categories -->
		{:else if activeTab === "groups"}
			{#if pluginStore.loadingRegistry}
				<div class="flex items-center justify-center py-8">
					<LoadingSpinner size="md" />
				</div>
			{:else if bundles.length === 0}
				<p class="py-4 text-center text-xs text-muted-foreground">
					No plugin bundles available. Check the Official tab for individual plugins.
				</p>
			{:else}
				<div class="space-y-2">
					{#each bundles as bundle (bundle.key)}
						<CardRoot gap={2}>
							<CardHeader>
								<CardTitle>
									<HStack gap={2}>
										<Icon name={bundle.icon} size="md" />
										{bundle.label}
									</HStack>
								</CardTitle>
								<CardAction>
									{#if isBundleInstalled(bundle)}
										<Badge variant="outline" size="xs">All installed</Badge>
									{:else}
										<Button
											variant="default"
											size="sm"
											disabled={installingBundle !== null}
											onclick={() => installBundle(bundle)}
										>
											{#if installingBundle === bundle.key}
												<LoadingSpinner size="sm" />
												Installing...
											{:else}
												Install all
											{/if}
										</Button>
									{/if}
								</CardAction>
							</CardHeader>
							<CardContent>
								<p class="text-[10px] text-muted-foreground">{bundle.description}</p>
								<div class="mt-2 space-y-1">
									{#each bundle.plugins as plugin (plugin.name)}
										<div class="flex items-center justify-between text-xs">
											<HStack gap={1.5}>
												<Icon name={plugin.icon ?? "puzzle"} size="sm" />
												<span>{plugin.displayName ?? plugin.display_name ?? plugin.name}</span>
											</HStack>
											{#if isInstalled(plugin.name)}
												<Badge variant="outline" size="xs">Installed</Badge>
											{/if}
										</div>
									{/each}
								</div>
							</CardContent>
						</CardRoot>
					{/each}
				</div>
			{/if}
		{/if}

		<!-- Manual Install -->
		<CardRoot gap={2}>
			<CardHeader>
				<CardTitle>Manual Install</CardTitle>
			</CardHeader>
			<CardContent>
				<p class="mb-2 text-[10px] text-muted-foreground">
					Enter a GitHub repo (owner/repo), a specific version (owner/repo@v0.2.0), or a local filesystem path.
				</p>
				<HStack gap={2}>
					<Input
						placeholder="orqastudio/orqastudio-plugin-claude"
						bind:value={manualSource}
						onkeydown={(e: KeyboardEvent) => { if (e.key === "Enter") installManual(); }}
					/>
					<Button
						variant="default"
						size="sm"
						disabled={installing !== null || !manualSource.trim()}
						onclick={installManual}
					>
						{#if installing === "manual"}
							<LoadingSpinner size="sm" />
						{:else}
							Install
						{/if}
					</Button>
				</HStack>
			</CardContent>
		</CardRoot>
	{/if}

	<!-- Error display -->
	{#if error}
		<CardRoot>
			<CardContent>
				<HStack gap={2} align="start">
					<Icon name="circle-x" size="sm" />
					<Text variant="caption">{error}</Text>
				</HStack>
			</CardContent>
		</CardRoot>
	{/if}
</div>

<!-- Conflict Resolution Dialog (rendered outside main layout for overlay) -->
{#if conflictDialog}
	<ConflictResolutionDialog
		conflicts={conflictDialog.conflicts}
		existingManifest={conflictDialog.existingManifest}
		newManifest={conflictDialog.newManifest}
		onResolve={handleConflictResolution}
		onCancel={handleConflictCancel}
	/>
{/if}
