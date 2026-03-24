<script lang="ts">
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { navigationStore, settingsStore, projectStore, artifactStore } = getStores();
	import { isArtifactGroup } from "@orqastudio/types";
	import type { NavigationItem } from "@orqastudio/types";
	import ActivityBarItem from "./ActivityBarItem.svelte";

	function resolveIconName(iconName: string | undefined): string {
		return iconName ?? "folder";
	}

	/**
	 * Look up the icon for a config entry from the navTree.
	 * For group entries, the navTree group is found by matching the first child's path prefix.
	 * For type entries, the navTree type is found by direct path match.
	 * Returns undefined if the navTree is not yet loaded or no match is found.
	 */
	function getNavTreeIcon(entryKey: string, entryPath?: string): string | undefined {
		const tree = artifactStore.navTree;
		if (!tree) return undefined;

		// Direct path match for type entries
		if (entryPath) {
			for (const group of tree.groups) {
				if (group.path === entryPath) return group.icon;
				for (const type of group.types) {
					if (type.path === entryPath) return type.icon;
				}
			}
		}

		// For group entries without a direct path, derive the group path from the first child
		const config = projectStore.artifactConfig;
		for (const cfgEntry of config) {
			if (isArtifactGroup(cfgEntry) && cfgEntry.key === entryKey && cfgEntry.children.length > 0) {
				const firstChildPath = cfgEntry.children[0].path;
				const groupPath = firstChildPath.split("/").slice(0, 2).join("/");
				for (const group of tree.groups) {
					if (group.path === groupPath) return group.icon;
				}
			}
		}

		return undefined;
	}

	/** Convert a config key to a human-readable label (mirrors Rust humanize_name). */
	function humanizeKey(key: string): string {
		return key
			.replace(/[-_]/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	const artifactConfig = $derived(projectStore.artifactConfig);
	const navItems = $derived(navigationStore.topLevelNavItems);

	/** Check if a NavigationItem or its children is active. */
	function isNavItemActive(item: NavigationItem): boolean {
		if (item.type === "group") {
			return navigationStore.activeGroup === item.key;
		}
		return navigationStore.activeActivity === item.key && navigationStore.activeGroup === null;
	}

	/** Handle click on a navigation item. */
	function handleNavClick(item: NavigationItem): void {
		if (item.type === "group") {
			navigationStore.setGroup(item.key);
		} else {
			navigationStore.activeGroup = null;
			navigationStore.setActivity(item.key);
		}
	}
</script>

<div class="flex w-12 flex-col items-center border-r border-border bg-muted/30 py-2">
	{#if navItems}
		<!-- New navigation model: render from navigation tree -->
		{#each navItems as item (item.key)}
			{#if !item.hidden}
				{#if item.key === "artifact-graph" || item.key === "settings"}
					<!-- These are rendered in the bottom section -->
				{:else}
					{@const entryLabel = item.label ?? humanizeKey(item.key)}
					<ActivityBarItem
						icon={item.icon}
						label={entryLabel}
						active={isNavItemActive(item)}
						onclick={() => handleNavClick(item)}
					/>
				{/if}
			{/if}
		{/each}

		<div class="flex-1"></div>

		<!-- Bottom items: Artifact Graph, Search, Settings -->
		{@const graphItem = navItems.find((i) => i.key === "artifact-graph")}
		{#if graphItem && !graphItem.hidden}
			<ActivityBarItem
				icon={graphItem.icon}
				label="Artifact Graph"
				active={navigationStore.activeActivity === "artifact-graph"}
				onclick={() => { navigationStore.activeGroup = null; navigationStore.setActivity("artifact-graph"); }}
			/>
		{/if}

		<ActivityBarItem
			icon="search"
			label="Search Artifacts (Ctrl+Space)"
			active={false}
			onclick={() => navigationStore.toggleSearch()}
		/>

		<div class="my-1 w-6">
			<Separator />
		</div>

		{@const settingsItem = navItems.find((i) => i.key === "settings")}
		{#if settingsItem && !settingsItem.hidden}
			<ActivityBarItem
				icon={settingsItem.icon}
				label="Project Settings"
				active={navigationStore.activeActivity === "settings"}
				onclick={() => { settingsStore.setActiveSection("project-general"); navigationStore.setActivity("settings"); }}
			/>
		{/if}
	{:else}
		<!-- Legacy mode: render from artifactConfig -->
		<!-- Project Dashboard -->
		<ActivityBarItem
			icon="layout-dashboard"
			label="Project Dashboard"
			active={navigationStore.activeActivity === "project"}
			onclick={() => navigationStore.setActivity("project")}
		/>

		{#if artifactConfig.length > 0}
			<div class="my-1 w-6">
				<Separator />
			</div>

			<!-- Config-driven artifact entries -->
			{#each artifactConfig as entry (entry.key)}
				{@const navIcon = isArtifactGroup(entry) ? getNavTreeIcon(entry.key) : getNavTreeIcon(entry.key, entry.path)}
				{@const iconName = resolveIconName(entry.icon ?? navIcon)}
				{@const entryLabel = entry.label ?? humanizeKey(entry.key)}
				{#if isArtifactGroup(entry)}
					<!-- Group entry — clicking activates the group -->
					<ActivityBarItem
						icon={iconName}
						label={entryLabel}
						active={navigationStore.activeGroup === entry.key}
						onclick={() => navigationStore.setGroup(entry.key)}
					/>
				{:else}
					<!-- Direct type entry — clicking activates the type directly -->
					<ActivityBarItem
						icon={iconName}
						label={entryLabel}
						active={navigationStore.activeActivity === entry.key && navigationStore.activeGroup === null}
						onclick={() => { navigationStore.activeGroup = null; navigationStore.setActivity(entry.key); }}
					/>
				{/if}
			{/each}
		{/if}

		<div class="flex-1"></div>

		<!-- Artifact Graph -->
		<ActivityBarItem
			icon="network"
			label="Artifact Graph"
			active={navigationStore.activeActivity === "artifact-graph"}
			onclick={() => { navigationStore.activeGroup = null; navigationStore.setActivity("artifact-graph"); }}
		/>

		<!-- Search -->
		<ActivityBarItem
			icon="search"
			label="Search Artifacts (Ctrl+Space)"
			active={false}
			onclick={() => navigationStore.toggleSearch()}
		/>

		<div class="my-1 w-6">
			<Separator />
		</div>

		<!-- Project Settings -->
		<ActivityBarItem
			icon="settings"
			label="Project Settings"
			active={navigationStore.activeActivity === "settings"}
			onclick={() => { settingsStore.setActiveSection("project-general"); navigationStore.setActivity("settings"); }}
		/>
	{/if}
</div>
