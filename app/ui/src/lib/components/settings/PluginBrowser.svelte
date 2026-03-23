<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardContent, CardAction } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { invoke, getStores, logger } from "@orqastudio/sdk";

	const log = logger("plugin-browser");
	import type { RegistrationConflict } from "@orqastudio/sdk";
	import type { PluginManifest } from "@orqastudio/types";
	import ConflictResolutionDialog from "./ConflictResolutionDialog.svelte";

	const { pluginRegistry } = getStores();

	// -----------------------------------------------------------------------
	// Types
	// -----------------------------------------------------------------------

	type Tab = "installed" | "official" | "community";
	type DetailView = { type: "installed" | "registry"; plugin: PluginEntry } | null;

	interface PluginEntry {
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
	let installed = $state<PluginEntry[]>([]);
	let official = $state<PluginEntry[]>([]);
	let community = $state<PluginEntry[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let manualSource = $state("");
	let installing = $state<string | null>(null);
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

	async function loadInstalled() {
		try {
			installed = await invoke<PluginEntry[]>("plugin_list_installed");
		} catch (err) {
			log.error("Failed to load installed plugins", { err });
			installed = [];
		}
	}

	async function loadRegistry(source: "official" | "community") {
		loading = true;
		error = null;
		try {
			const result = await invoke<{ plugins: PluginEntry[] }>("plugin_registry_list", { source });
			if (source === "official") official = result.plugins;
			else community = result.plugins;
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			loading = false;
		}
	}

	async function handleTabChange(tab: Tab) {
		activeTab = tab;
		detailView = null;
		if (tab === "official" && official.length === 0) await loadRegistry("official");
		if (tab === "community" && community.length === 0) await loadRegistry("community");
	}

	// -----------------------------------------------------------------------
	// Install / Uninstall
	// -----------------------------------------------------------------------

	async function installFromRegistry(plugin: PluginEntry) {
		if (!plugin.repo) return;
		installing = plugin.name;
		error = null;
		try {
			await invoke("plugin_install_github", { repo: plugin.repo });

			// Read the installed manifest and check for conflicts before registering
			const manifest = await invoke<PluginManifest>("plugin_get_manifest", { name: plugin.name });
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

	function handleConflictCancel() {
		if (conflictDialog) {
			// Uninstall the plugin that was downloaded but couldn't register
			void uninstallPlugin(conflictDialog.pendingPlugin.name);
		}
		conflictDialog = null;
	}

	async function installManual() {
		if (!manualSource.trim()) return;
		installing = "manual";
		error = null;
		try {
			const source = manualSource.trim();
			if (source.includes("/") && !source.includes("\\") && !source.includes(":")) {
				// GitHub repo format: owner/repo or owner/repo@version
				const [repo, version] = source.split("@");
				await invoke("plugin_install_github", { repo, version: version ?? null });
			} else {
				await invoke("plugin_install_local", { path: source });
			}
			manualSource = "";
			await loadInstalled();
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		} finally {
			installing = null;
		}
	}

	async function uninstallPlugin(name: string) {
		error = null;
		try {
			await invoke("plugin_uninstall", { name });
			await loadInstalled();
			if (detailView?.plugin.name === name) detailView = null;
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
		}
	}

	// -----------------------------------------------------------------------
	// Detail View
	// -----------------------------------------------------------------------

	async function showDetail(plugin: PluginEntry, type: "installed" | "registry") {
		detailView = { type, plugin };
		detailManifest = null;

		if (type === "installed") {
			detailLoading = true;
			try {
				detailManifest = await invoke<PluginManifestData>("plugin_get_manifest", { name: plugin.name });
			} catch (err) {
				log.error("Failed to load plugin manifest for detail view", { pluginName: plugin.name, err });
				detailManifest = null;
			} finally {
				detailLoading = false;
			}
		}
	}

	function closeDetail() {
		detailView = null;
		detailManifest = null;
	}

	// -----------------------------------------------------------------------
	// Helpers
	// -----------------------------------------------------------------------

	function displayName(plugin: PluginEntry): string {
		return plugin.displayName ?? plugin.display_name ?? plugin.name;
	}

	function isInstalled(name: string): boolean {
		return installed.some((p) => p.name === name);
	}

	function installedVersion(name: string): string | undefined {
		return installed.find((p) => p.name === name)?.version;
	}
</script>

<div class="space-y-4">
	<!-- Header -->
	<CardRoot>
		<CardHeader>
			<CardTitle class="text-sm font-semibold">
				<div class="flex items-center gap-2">
					<Icon name="puzzle" size="md" />
					Plugins
				</div>
			</CardTitle>
			<CardAction>
				<Badge variant="outline" class="text-[10px] px-1.5 py-0">
					{installed.length} installed
				</Badge>
			</CardAction>
		</CardHeader>
	</CardRoot>

	{#if detailView}
		<!-- Detail View -->
		<div class="space-y-3">
			<button
				class="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors"
				onclick={closeDetail}
			>
				<Icon name="arrow-left" size="sm" />
				Back to {activeTab}
			</button>

			<CardRoot>
				<CardHeader>
					<CardTitle class="text-sm font-semibold">
						<div class="flex items-center gap-2">
							<Icon name={detailView.plugin.icon ?? "puzzle"} size="md" />
							{displayName(detailView.plugin)}
						</div>
					</CardTitle>
					<CardAction>
						{#if detailView.type === "installed"}
							<Button
								variant="ghost"
								size="sm"
								class="h-7 px-2 text-xs text-destructive"
								onclick={() => detailView && uninstallPlugin(detailView.plugin.name)}
							>
								Uninstall
							</Button>
						{:else if !isInstalled(detailView.plugin.name)}
							<Button
								variant="default"
								size="sm"
								class="h-7 px-3 text-xs"
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
							<Badge variant="outline" class="text-[10px]">Installed</Badge>
						{/if}
					</CardAction>
				</CardHeader>
				<CardContent>
					<p class="text-xs text-muted-foreground">{detailView.plugin.description ?? "No description"}</p>
					<div class="mt-2 flex gap-2 text-[10px] text-muted-foreground">
						{#if detailView.plugin.version}
							<span>v{detailView.plugin.version}</span>
						{/if}
						{#if detailView.plugin.repo}
							<span>{detailView.plugin.repo}</span>
						{/if}
						{#if detailView.plugin.source}
							<span>{detailView.plugin.source}</span>
						{/if}
					</div>
					{#if detailView.plugin.capabilities?.length}
						<div class="mt-2 flex flex-wrap gap-1">
							{#each detailView.plugin.capabilities as cap}
								<Badge variant="outline" class="text-[9px] px-1.5 py-0">{cap}</Badge>
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
					<CardRoot class="gap-1">
						<CardHeader class="pb-1">
							<CardTitle class="text-xs font-semibold">Artifact Schemas ({detailManifest.provides.schemas.length})</CardTitle>
						</CardHeader>
						<CardContent class="pt-0">
							<div class="space-y-1">
								{#each detailManifest.provides.schemas as schema}
									<div class="flex items-center gap-2 text-xs">
										<Icon name={schema.icon} size="sm" />
										<span class="font-medium">{schema.label}</span>
										<span class="text-muted-foreground">({schema.key})</span>
									</div>
								{/each}
							</div>
						</CardContent>
					</CardRoot>
				{/if}

				{#if detailManifest.provides.relationships.length > 0}
					<CardRoot class="gap-1">
						<CardHeader class="pb-1">
							<CardTitle class="text-xs font-semibold">Relationships ({detailManifest.provides.relationships.length})</CardTitle>
						</CardHeader>
						<CardContent class="pt-0">
							<div class="space-y-1">
								{#each detailManifest.provides.relationships as rel}
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
					<CardRoot class="gap-1">
						<CardHeader class="pb-1">
							<CardTitle class="text-xs font-semibold">Views ({detailManifest.provides.views.length})</CardTitle>
						</CardHeader>
						<CardContent class="pt-0">
							<div class="space-y-1">
								{#each detailManifest.provides.views as view}
									<div class="flex items-center gap-2 text-xs">
										<Icon name={view.icon} size="sm" />
										<span>{view.label}</span>
									</div>
								{/each}
							</div>
						</CardContent>
					</CardRoot>
				{/if}

				{#if detailManifest.provides.widgets.length > 0}
					<CardRoot class="gap-1">
						<CardHeader class="pb-1">
							<CardTitle class="text-xs font-semibold">Widgets ({detailManifest.provides.widgets.length})</CardTitle>
						</CardHeader>
						<CardContent class="pt-0">
							<div class="space-y-1">
								{#each detailManifest.provides.widgets as widget}
									<div class="flex items-center gap-2 text-xs">
										<Icon name={widget.icon} size="sm" />
										<span>{widget.label}</span>
									</div>
								{/each}
							</div>
						</CardContent>
					</CardRoot>
				{/if}

				{#if detailManifest.provides.cli_tools.length > 0 || detailManifest.provides.hooks.length > 0}
					<CardRoot class="gap-1">
						<CardHeader class="pb-1">
							<CardTitle class="text-xs font-semibold">Backend Capabilities</CardTitle>
						</CardHeader>
						<CardContent class="pt-0">
							<div class="space-y-1 text-xs">
								{#each detailManifest.provides.cli_tools as tool}
									<div class="flex items-center gap-2">
										<Icon name="terminal" size="sm" />
										<span>{tool.label}</span>
									</div>
								{/each}
								{#each detailManifest.provides.hooks as hook}
									<div class="flex items-center gap-2">
										<Icon name="webhook" size="sm" />
										<span>{hook.key}</span>
										<span class="text-muted-foreground">({hook.event})</span>
									</div>
								{/each}
							</div>
						</CardContent>
					</CardRoot>
				{/if}
			{/if}
		</div>
	{:else}
		<!-- Tab bar -->
		<div class="flex gap-1 rounded-md border border-border p-1">
			{#each ["installed", "official", "community"] as tab}
				<button
					class="flex-1 rounded px-3 py-1.5 text-xs font-medium transition-colors"
					class:bg-primary={activeTab === tab}
					class:text-primary-foreground={activeTab === tab}
					class:text-muted-foreground={activeTab !== tab}
					onclick={() => handleTabChange(tab as Tab)}
				>
					{tab.charAt(0).toUpperCase() + tab.slice(1)}
				</button>
			{/each}
		</div>

		<!-- Installed tab -->
		{#if activeTab === "installed"}
			<div class="space-y-2">
				{#each installed as plugin}
					<button
						class="w-full text-left"
						onclick={() => showDetail(plugin, "installed")}
					>
						<CardRoot class="gap-2 transition-colors hover:bg-accent/30 cursor-pointer">
							<CardContent class="py-3">
								<div class="flex items-center justify-between">
									<div>
										<p class="text-xs font-medium">{displayName(plugin)}</p>
										<p class="text-[10px] text-muted-foreground">
											v{plugin.version ?? "?"} — {plugin.source ?? "local"}
										</p>
										{#if plugin.description}
											<p class="mt-1 text-[10px] text-muted-foreground line-clamp-1">{plugin.description}</p>
										{/if}
									</div>
									<div class="flex items-center gap-2">
										<Icon name="chevron-right" size="sm" />
									</div>
								</div>
							</CardContent>
						</CardRoot>
					</button>
				{:else}
					<p class="py-4 text-center text-xs text-muted-foreground">
						No plugins installed yet. Browse the Official tab or use Manual Install below.
					</p>
				{/each}
			</div>

		<!-- Official tab -->
		{:else if activeTab === "official"}
			{#if loading}
				<div class="flex items-center justify-center py-8">
					<LoadingSpinner size="md" />
				</div>
			{:else}
				<div class="space-y-2">
					{#each official as plugin}
						<button
							class="w-full text-left"
							onclick={() => showDetail(plugin, "registry")}
						>
							<CardRoot class="gap-2 transition-colors hover:bg-accent/30 cursor-pointer">
								<CardContent class="py-3">
									<div class="flex items-center justify-between">
										<div>
											<div class="flex items-center gap-2">
												<Icon name={plugin.icon ?? "puzzle"} size="sm" />
												<p class="text-xs font-medium">{displayName(plugin)}</p>
												{#if isInstalled(plugin.name)}
													<Badge variant="outline" class="text-[9px] px-1 py-0">Installed</Badge>
												{/if}
											</div>
											<p class="mt-1 text-[10px] text-muted-foreground line-clamp-1">{plugin.description}</p>
											{#if plugin.capabilities?.length}
												<div class="mt-1 flex gap-1">
													{#each plugin.capabilities as cap}
														<Badge variant="outline" class="text-[9px] px-1 py-0">{cap}</Badge>
													{/each}
												</div>
											{/if}
										</div>
										<div class="flex items-center gap-2 shrink-0">
											{#if !isInstalled(plugin.name)}
												<Button
													variant="default"
													size="sm"
													class="h-7 px-3 text-xs"
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
									</div>
								</CardContent>
							</CardRoot>
						</button>
					{:else}
						<p class="py-4 text-center text-xs text-muted-foreground">
							No official plugins available yet.
						</p>
					{/each}
				</div>
			{/if}

		<!-- Community tab -->
		{:else if activeTab === "community"}
			{#if loading}
				<div class="flex items-center justify-center py-8">
					<LoadingSpinner size="md" />
				</div>
			{:else}
				<div class="space-y-2">
					{#each community as plugin}
						<button
							class="w-full text-left"
							onclick={() => showDetail(plugin, "registry")}
						>
							<CardRoot class="gap-2 transition-colors hover:bg-accent/30 cursor-pointer">
								<CardContent class="py-3">
									<div class="flex items-center justify-between">
										<div>
											<div class="flex items-center gap-2">
												<Icon name={plugin.icon ?? "puzzle"} size="sm" />
												<p class="text-xs font-medium">{displayName(plugin)}</p>
												<Badge variant="secondary" class="text-[9px] px-1 py-0">Community</Badge>
												{#if isInstalled(plugin.name)}
													<Badge variant="outline" class="text-[9px] px-1 py-0">Installed</Badge>
												{/if}
											</div>
											<p class="mt-1 text-[10px] text-muted-foreground line-clamp-1">{plugin.description}</p>
										</div>
										<div class="flex items-center gap-2 shrink-0">
											{#if !isInstalled(plugin.name)}
												<Button
													variant="outline"
													size="sm"
													class="h-7 px-3 text-xs"
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
									</div>
								</CardContent>
							</CardRoot>
						</button>
					{:else}
						<p class="py-4 text-center text-xs text-muted-foreground">
							No community plugins available yet.
						</p>
					{/each}
				</div>
			{/if}
		{/if}

		<!-- Manual Install -->
		<CardRoot class="gap-2">
			<CardHeader class="pb-2">
				<CardTitle class="text-xs font-semibold">Manual Install</CardTitle>
			</CardHeader>
			<CardContent class="pt-0">
				<p class="mb-2 text-[10px] text-muted-foreground">
					Enter a GitHub repo (owner/repo), a specific version (owner/repo@v0.2.0), or a local filesystem path.
				</p>
				<div class="flex gap-2">
					<input
						type="text"
						class="flex-1 rounded border border-border bg-background px-3 py-1.5 text-xs placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
						placeholder="orqastudio/orqastudio-plugin-claude"
						bind:value={manualSource}
						onkeydown={(e: KeyboardEvent) => { if (e.key === "Enter") installManual(); }}
					/>
					<Button
						variant="default"
						size="sm"
						class="h-8 px-3 text-xs"
						disabled={installing !== null || !manualSource.trim()}
						onclick={installManual}
					>
						{#if installing === "manual"}
							<LoadingSpinner size="sm" />
						{:else}
							Install
						{/if}
					</Button>
				</div>
			</CardContent>
		</CardRoot>
	{/if}

	<!-- Error display -->
	{#if error}
		<CardRoot class="border-destructive/50 bg-destructive/5">
			<CardContent class="py-3">
				<div class="flex items-start gap-2">
					<Icon name="circle-x" size="sm" />
					<p class="text-xs text-destructive">{error}</p>
				</div>
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
