<script lang="ts">
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import BotIcon from "@lucide/svelte/icons/bot";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import type { Component } from "svelte";

	let { items }: { items: string[] } = $props();

	const activityIcon: Record<string, Component> = {
		docs: FileTextIcon,
		agents: BotIcon,
		rules: ShieldIcon,
		skills: ZapIcon,
		hooks: GitBranchIcon,
	};

	const Icon = $derived(activityIcon[navigationStore.activeActivity] ?? FileTextIcon);

	function handleHome() {
		if (navigationStore.activeActivity === "docs") {
			navigationStore.openArtifact("README", []);
		} else {
			navigationStore.closeArtifact();
		}
	}
</script>

<nav class="flex items-center gap-1 text-sm">
	<button
		class="flex items-center text-muted-foreground hover:text-foreground"
		onclick={handleHome}
	>
		<Icon class="h-3.5 w-3.5" />
	</button>

	{#each items as item, index}
		<ChevronRightIcon class="h-3 w-3 text-muted-foreground" />
		{#if index === items.length - 1}
			<span class="font-medium text-foreground">{item}</span>
		{:else}
			<button
				class="text-muted-foreground hover:text-foreground"
				onclick={handleHome}
			>
				{item}
			</button>
		{/if}
	{/each}
</nav>
