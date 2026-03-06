<script lang="ts">
	import DocTreeNav from "$lib/components/navigation/DocTreeNav.svelte";
	import ArtifactListNav from "$lib/components/navigation/ArtifactListNav.svelte";
	import SettingsCategoryNav from "$lib/components/navigation/SettingsCategoryNav.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
</script>

<div class="flex w-[200px] flex-col overflow-hidden border-r border-border bg-muted/10">
	<!-- Panel header — fixed height matched to breadcrumb bar -->
	<div class="flex h-10 items-center border-b border-border px-3">
		<h2 class="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
			{#if navigationStore.activeActivity === "docs"}
				Docs
			{:else if navigationStore.activeActivity === "research"}
				Research
			{:else if navigationStore.activeActivity === "plans"}
				Plans
			{:else if navigationStore.activeActivity === "agents"}
				Agents
			{:else if navigationStore.activeActivity === "rules"}
				Rules
			{:else if navigationStore.activeActivity === "skills"}
				Skills
			{:else if navigationStore.activeActivity === "hooks"}
				Hooks
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
		{#if navigationStore.activeActivity === "docs"}
			<DocTreeNav mode="docs" />
		{:else if navigationStore.activeActivity === "research"}
			<DocTreeNav mode="research" />
		{:else if navigationStore.activeActivity === "plans"}
			<DocTreeNav mode="plans" />
		{:else if navigationStore.activeActivity === "agents" || navigationStore.activeActivity === "rules" || navigationStore.activeActivity === "skills" || navigationStore.activeActivity === "hooks"}
			<ArtifactListNav category={navigationStore.activeActivity} />
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
