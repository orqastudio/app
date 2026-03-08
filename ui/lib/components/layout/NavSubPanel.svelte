<script lang="ts">
	import DocTreeNav from "$lib/components/navigation/DocTreeNav.svelte";
	import ArtifactListNav from "$lib/components/navigation/ArtifactListNav.svelte";
	import SettingsCategoryNav from "$lib/components/navigation/SettingsCategoryNav.svelte";
	import GroupSubPanel from "$lib/components/navigation/GroupSubPanel.svelte";
	import { navigationStore, type ActivityGroup } from "$lib/stores/navigation.svelte";

	const GROUP_LABELS: Record<ActivityGroup, string> = {
		documentation: "Documentation",
		planning: "Planning",
		team: "Team",
		governance: "Governance",
	};
</script>

<div class="flex w-[200px] flex-col overflow-hidden border-r border-border bg-muted/10">
	<!-- Panel header — fixed height matched to breadcrumb bar -->
	<div class="flex h-10 items-center border-b border-border px-3">
		<h2 class="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
			{#if navigationStore.activeGroup !== null}
				{GROUP_LABELS[navigationStore.activeGroup]}
			{:else if navigationStore.activeActivity === "settings"}
				Project Settings
			{:else if navigationStore.activeActivity === "configure"}
				Configuration
			{:else if navigationStore.activeActivity === "chat"}
				Sessions
			{:else}
				{navigationStore.activeActivity}
			{/if}
		</h2>
	</div>

	<!-- Panel content -->
	<div class="flex-1 overflow-hidden">
		{#if navigationStore.activeGroup !== null}
			<GroupSubPanel group={navigationStore.activeGroup} />
		{:else if navigationStore.activeActivity === "settings"}
			<SettingsCategoryNav mode="project" />
		{:else if navigationStore.activeActivity === "configure"}
			<SettingsCategoryNav mode="app" />
		{:else if navigationStore.activeActivity === "chat"}
			<div class="flex h-full items-center justify-center p-4 text-center text-xs text-muted-foreground">
				Session list will be available in a future update.
			</div>
		{/if}
	</div>
</div>
