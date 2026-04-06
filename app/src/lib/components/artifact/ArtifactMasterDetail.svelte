<!-- Master-detail layout: file browser on the left, artifact viewer on the right. Auto-loads category README when nothing is selected. -->
<script lang="ts">
	import ArtifactNav from "$lib/components/navigation/ArtifactNav.svelte";
	import ArtifactViewer from "./ArtifactViewer.svelte";
	import { HStack, Box, Text, Sidebar } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";
	import type { ActivityView } from "@orqastudio/sdk";

	const { navigationStore } = getStores();

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

<!-- HStack fills full height; Sidebar provides the fixed w-60 with a right border. -->
<HStack gap={0} height="full" align="stretch">
	<!-- File Browser -->
	<Sidebar width="md">
		<ArtifactNav category={activity} />
	</Sidebar>

	<!-- Viewer -->
	<Box flex={1} minWidth={0}>
		{#if hasSelection}
			<ArtifactViewer />
		{:else}
			<HStack justify="center" align="center" height="full">
				<Text variant="body-muted">Select an item to view it</Text>
			</HStack>
		{/if}
	</Box>
</HStack>
