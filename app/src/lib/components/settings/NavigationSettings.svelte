<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import { Icon, ScrollArea } from "@orqastudio/svelte-components/pure";
	import type { NavigationItem } from "@orqastudio/types";

	const { projectStore, pluginRegistry } = getStores();

	const navigation = $derived(projectStore.projectSettings?.navigation ?? []);

	function humanizeKey(key: string): string {
		return key
			.replace(/[-_]/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	function resolveLabel(item: NavigationItem): string {
		if (item.label) return item.label;
		if (item.type === "plugin" && item.pluginSource) {
			const resolved = pluginRegistry.resolveNavigationItem(item);
			return resolved.label;
		}
		return humanizeKey(item.key);
	}

	function typeLabel(type: NavigationItem["type"]): string {
		switch (type) {
			case "builtin": return "Built-in";
			case "plugin": return "Plugin";
			case "group": return "Group";
			default: return type;
		}
	}
</script>

<div class="space-y-6 p-6">
	<div>
		<h2 class="text-lg font-semibold">Navigation</h2>
		<p class="text-sm text-muted-foreground">
			Manage the navigation tree. Items are displayed in the order shown below.
		</p>
	</div>

	{#if navigation.length === 0}
		<div class="rounded-md border border-dashed p-6 text-center text-sm text-muted-foreground">
			<p>No navigation tree configured.</p>
			<p class="mt-1">Add a <code class="rounded bg-muted px-1">navigation</code> array to your project.json to enable the new navigation model.</p>
		</div>
	{:else}
		<ScrollArea class="max-h-[600px]">
			<div class="space-y-1">
				{#each navigation as item (item.key)}
					<div class="rounded-md border border-border bg-card p-3">
						<div class="flex items-center gap-3">
							<Icon name={item.icon} size="md" />
							<div class="min-w-0 flex-1">
								<div class="flex items-center gap-2">
									<span class="text-sm font-medium">{resolveLabel(item)}</span>
									<span class="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">
										{typeLabel(item.type)}
									</span>
									{#if item.pluginSource}
										<span class="text-xs text-muted-foreground">
											from {item.pluginSource}
										</span>
									{/if}
								</div>
								<div class="text-xs text-muted-foreground">{item.key}</div>
							</div>
							{#if item.hidden}
								<span class="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">Hidden</span>
							{/if}
						</div>

						{#if item.type === "group" && item.children}
							<div class="ml-8 mt-2 space-y-1">
								{#each item.children as child (child.key)}
									<div class="flex items-center gap-2 rounded px-2 py-1.5 text-sm {child.hidden ? 'opacity-50' : ''}">
										<Icon name={child.icon} size="sm" />
										<span>{resolveLabel(child)}</span>
										<span class="rounded bg-muted px-1 py-0.5 text-xs text-muted-foreground">
											{typeLabel(child.type)}
										</span>
										{#if child.pluginSource}
											<span class="text-xs text-muted-foreground">
												from {child.pluginSource}
											</span>
										{/if}
										{#if child.hidden}
											<span class="rounded bg-muted px-1 py-0.5 text-xs text-muted-foreground">Hidden</span>
										{/if}
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</ScrollArea>
	{/if}

	<div class="rounded-md border border-border bg-muted/30 p-4">
		<h3 class="text-sm font-medium">Installed Plugins</h3>
		<div class="mt-2 space-y-2">
			{#each pluginRegistry.pluginNames as name (name)}
				{@const plugin = pluginRegistry.getPlugin(name)}
				{#if plugin}
					<div class="flex items-center gap-3 text-sm">
						<div class="h-2 w-2 rounded-full bg-green-500"></div>
						<div>
							<span class="font-medium">{plugin.manifest.displayName ?? plugin.manifest.name}</span>
							<span class="text-muted-foreground">v{plugin.manifest.version}</span>
						</div>
						<span class="text-xs text-muted-foreground">
							{plugin.manifest.provides.schemas.length} schemas,
							{plugin.manifest.provides.views.length} views,
							{plugin.manifest.provides.relationships.length} relationships
						</span>
					</div>
				{/if}
			{/each}
			{#if pluginRegistry.pluginNames.length === 0}
				<p class="text-sm text-muted-foreground">No plugins installed.</p>
			{/if}
		</div>
	</div>
</div>
