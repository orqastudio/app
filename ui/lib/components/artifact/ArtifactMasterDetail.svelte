<script lang="ts">
	import ArtifactNav from "$lib/components/navigation/ArtifactNav.svelte";
	import ArtifactViewer from "./ArtifactViewer.svelte";
	import { navigationStore, type ActivityView } from "$lib/stores/navigation.svelte";

	let { activity }: { activity: ActivityView } = $props();

	/** README path for each category — shown when nothing is explicitly selected. */
	const README_PATHS: Partial<Record<ActivityView, string>> = {
		milestones: ".orqa/milestones/README.md",
		epics: ".orqa/epics/README.md",
		tasks: ".orqa/tasks/README.md",
		ideas: ".orqa/ideas/README.md",
		decisions: ".orqa/decisions/README.md",
		lessons: ".orqa/lessons/README.md",
		agents: ".orqa/agents/README.md",
		rules: ".claude/rules/README.md",
		skills: ".claude/skills/README.md",
		hooks: ".claude/hooks/README.md",
		docs: "docs/README.md",
		research: ".orqa/research/README.md",
		plans: ".orqa/plans/README.md",
	};

	const hasSelection = $derived(navigationStore.selectedArtifactPath !== null);

	/** When the activity changes and nothing is selected, auto-load the category README. */
	$effect(() => {
		const act = activity;
		if (navigationStore.selectedArtifactPath !== null) return;
		const readmePath = README_PATHS[act];
		if (readmePath) {
			navigationStore.openArtifact(readmePath, []);
		}
	});
</script>

<div class="flex h-full">
	<!-- File Browser (240px) -->
	<div class="w-60 shrink-0 overflow-hidden border-r border-border">
		<ArtifactNav category={activity} />
	</div>

	<!-- Viewer -->
	<div class="min-w-0 flex-1 overflow-hidden">
		{#if hasSelection}
			<ArtifactViewer />
		{:else}
			<div class="flex h-full items-center justify-center text-sm text-muted-foreground">
				Select an item to view it
			</div>
		{/if}
	</div>
</div>
