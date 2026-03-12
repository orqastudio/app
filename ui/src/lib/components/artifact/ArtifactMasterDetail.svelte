<script lang="ts">
	import ArtifactNav from "$lib/components/navigation/ArtifactNav.svelte";
	import ArtifactViewer from "./ArtifactViewer.svelte";
	import { navigationStore, type ActivityView } from "$lib/stores/navigation.svelte";

	let { activity }: { activity: ActivityView } = $props();

	/**
	 * Derive the README path for the current activity from the navTree.
	 * The folder structure IS the config — no hardcoded paths needed.
	 */
	const readmePath = $derived.by(() => {
		const navType = navigationStore.getNavType(activity);
		if (navType) {
			return `${navType.path}/README.md`;
		}
		return null;
	});

	const hasSelection = $derived(navigationStore.selectedArtifactPath !== null);

	/** When the activity changes and nothing is selected, auto-load the category README. */
	$effect(() => {
		void activity; // track activity changes to trigger re-evaluation
		if (navigationStore.selectedArtifactPath !== null) return;
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
