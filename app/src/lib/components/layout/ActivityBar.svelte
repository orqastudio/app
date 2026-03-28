<script lang="ts">
	// Activity bar component — renders top-level navigation items derived from
	// PLATFORM_NAVIGATION merged with plugin defaultNavigation. The navigation
	// tree is always available once a project is open; this component does not
	// fall back to legacy artifact config rendering.
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { navigationStore, settingsStore } = getStores();
	import type { NavigationItem } from "@orqastudio/types";
	import ActivityBarItem from "./ActivityBarItem.svelte";

	/** Convert a config key to a human-readable label (mirrors Rust humanize_name). */
	function humanizeKey(key: string): string {
		return key
			.replace(/[-_]/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

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
		<!-- Render main nav items, skipping bottom fixed items -->
		{#each navItems as item (item.key)}
			{#if !item.hidden}
				{#if item.key === "artifact-graph" || item.key === "plugins" || item.key === "settings"}
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

		<!-- Bottom items: Artifact Graph, Search, Plugins, Settings -->
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

		{@const pluginsItem = navItems.find((i) => i.key === "plugins")}
		{#if pluginsItem && !pluginsItem.hidden}
			<ActivityBarItem
				icon={pluginsItem.icon}
				label="Plugins"
				active={navigationStore.activeActivity === "plugins"}
				onclick={() => { navigationStore.activeGroup = null; navigationStore.setActivity("plugins"); }}
			/>
		{/if}

		{@const settingsItem = navItems.find((i) => i.key === "settings")}
		{#if settingsItem && !settingsItem.hidden}
			<ActivityBarItem
				icon={settingsItem.icon}
				label="Project Settings"
				active={navigationStore.activeActivity === "settings"}
				onclick={() => { settingsStore.setActiveSection("project-general"); navigationStore.setActivity("settings"); }}
			/>
		{/if}
	{/if}
</div>
