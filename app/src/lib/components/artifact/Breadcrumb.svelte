<!-- Artifact breadcrumb navigation using the ORQA Breadcrumb primitive. Home navigates to the artifact list; intermediate segments return to list root. -->
<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import { Breadcrumb, type BreadcrumbItem } from "@orqastudio/svelte-components/pure";

	const { navigationStore } = getStores();

	let { items }: { items: string[] } = $props();

	/** Navigate home by closing the artifact viewer and returning to the list. */
	function handleHome() {
		navigationStore.closeArtifact();
	}

	/**
	 * Navigate to an intermediate breadcrumb at the given index.
	 * items[0] is the section label, items[1..n-1] are folder segments,
	 * items[n-1] is the leaf (non-clickable). Clicking a folder segment
	 * closes the artifact viewer and returns to the list.
	 * @param index - The index of the clicked breadcrumb segment.
	 */
	function handleSegmentClick(index: number) {
		// Only the first segment (section label) has a meaningful navigation target:
		// return to the artifact list for this category.
		// Deeper folder segments don't correspond to selectable routes in the current
		// navigation model, so they also return to the list root.
		if (index < items.length - 1) {
			navigationStore.closeArtifact();
		}
	}

	/** Map string items to BreadcrumbItem objects with click handlers for non-leaf segments. */
	const breadcrumbItems = $derived(
		items.map((item, index): BreadcrumbItem => {
			if (index < items.length - 1) {
				return { label: item, onClick: () => handleSegmentClick(index) };
			}
			return item;
		}),
	);
</script>

<Breadcrumb items={breadcrumbItems} onHome={handleHome} />
