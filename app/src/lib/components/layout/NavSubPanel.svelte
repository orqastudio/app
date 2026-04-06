<script lang="ts">
	// Navigation sub-panel — secondary panel rendered to the right of the activity bar.
	// Shows context-sensitive content: settings nav, group children, artifact list,
	// or plugin browser depending on the active activity.
	import { Caption, Box, Center, Text, Panel } from "@orqastudio/svelte-components/pure";
	import SettingsCategoryNav from "$lib/components/navigation/SettingsCategoryNav.svelte";
	import GroupSubPanel from "$lib/components/navigation/GroupSubPanel.svelte";
	import ArtifactNav from "$lib/components/navigation/ArtifactNav.svelte";
	import { getStores } from "@orqastudio/sdk";

	const { navigationStore } = getStores();
</script>

<div class="border-border bg-muted/10 flex w-[200px] flex-col overflow-hidden border-r">
	<!-- Panel header — fixed height matched to breadcrumb bar -->
	<div class="border-border flex h-10 items-center border-b px-3">
		<Text variant="overline-muted">
			{#if navigationStore.activeGroup !== null}
				{navigationStore.getLabelForKey(navigationStore.activeGroup)}
			{:else if navigationStore.activeActivity === "settings"}
				Project Settings
			{:else if navigationStore.activeActivity === "chat"}
				Sessions
			{:else if navigationStore.activeActivity === "plugins"}
				Plugins
			{:else}
				{navigationStore.getLabelForKey(navigationStore.activeActivity)}
			{/if}
		</Text>
	</div>

	<!-- Panel content -->
	<Box flex={1}>
		{#if navigationStore.activeGroup !== null}
			<GroupSubPanel group={navigationStore.activeGroup} />
		{:else if navigationStore.activeActivity === "settings"}
			<SettingsCategoryNav mode="project" />
		{:else if navigationStore.activeActivity === "chat"}
			<Panel padding="normal"
				><Center full>
					<Caption>Session list will be available in a future update.</Caption>
				</Center></Panel
			>
		{:else if navigationStore.activeActivity === "plugins"}
			<!-- Plugin browser is displayed in the main explorer area. -->
			<Panel padding="normal"
				><Center full>
					<Caption>Select a tab in the plugin browser.</Caption>
				</Center></Panel
			>
		{:else if navigationStore.isArtifactActivity}
			<ArtifactNav category={navigationStore.activeActivity} />
		{/if}
	</Box>
</div>
