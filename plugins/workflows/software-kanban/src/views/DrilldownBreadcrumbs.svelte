<!-- DrilldownBreadcrumbs: renders roadmap drill-down navigation using the Breadcrumb primitive. -->
<script lang="ts">
	import { Breadcrumb, type BreadcrumbItem } from "@orqastudio/svelte-components/pure";

	type BreadcrumbItemDef = {
		label: string;
		onClick: () => void;
	};

	let { items }: { items: BreadcrumbItemDef[] } = $props();

	// Map to the Breadcrumb primitive's item shape.
	// The first item acts as the home link (showHome + onHome), remaining items are segments.
	const breadcrumbItems = $derived<BreadcrumbItem[]>(
		items.slice(1).map((item) => ({ label: item.label, onClick: item.onClick })),
	);

	const homeItem = $derived(items[0]);
</script>

<Breadcrumb items={breadcrumbItems} showHome={true} onHome={homeItem?.onClick} maxWidth="240px" />
