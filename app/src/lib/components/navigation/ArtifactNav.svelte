<script lang="ts">
	import {
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleGroupHeader,
		TreeCollapsibleTrigger,
		EmptyState,
		LoadingSpinner,
		ErrorDisplay,
		Caption,
		Stack,
		Box,
		Center,
		Panel,
		ScrollArea,
		TreeIndent,
	} from "@orqastudio/svelte-components/pure";
	import { ArtifactListItem } from "@orqastudio/svelte-components/connected";
	import ArtifactToolbar from "$lib/components/navigation/ArtifactToolbar.svelte";
	import { getStores } from "@orqastudio/sdk";
	import type { ActivityView } from "@orqastudio/sdk";

	const { artifactStore, navigationStore } = getStores();
	import type { DocNode, ArtifactViewState, SortConfig } from "@orqastudio/types";
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { applyFilters, applySort, applyGrouping } from "$lib/utils/artifact-view";
	import { SvelteMap } from "svelte/reactivity";

	/**
	 * Return the icon name for a directory node, falling back to "folder".
	 * @param iconName - The icon name from the node definition, which may be null or undefined.
	 * @returns The provided icon name, or "folder" as a fallback.
	 */
	function resolveDirectoryIcon(iconName: string | null | undefined): string {
		return iconName ?? "folder";
	}

	let { category }: { category: ActivityView } = $props();

	/** View states keyed by category path (one per artifact type). */
	const viewStates = new SvelteMap<string, ArtifactViewState>();

	/**
	 * Return the persisted view state for a category, initializing from nav config defaults if absent.
	 * @param cat - The category key (e.g. "task", "epic") whose view state is needed.
	 * @returns The current view state (sort, filters, group) for the given category.
	 */
	function getViewState(cat: string): ArtifactViewState {
		if (!viewStates.has(cat)) {
			// Initialize from navigation config defaults
			const navConfig = navigationStore.getNavType(cat as ActivityView)?.navigation_config;
			const defaults = navConfig?.defaults;
			viewStates.set(cat, {
				sort: defaults?.sort ?? { field: "title", direction: "asc" },
				filters: defaults?.filters ?? {},
				group: defaults?.group ?? null,
			});
		}
		return viewStates.get(cat)!;
	}

	/** Find the NavType for this category in the navTree. */
	const currentNavType = $derived(navigationStore.getNavType(category));

	/** Nodes for this category — either from navTree or empty. */
	const allNodes = $derived(currentNavType ? currentNavType.nodes : []);

	/** Label for this category. */
	const categoryLabel = $derived(currentNavType?.label ?? navigationStore.getLabelForKey(category));

	/** Whether data is still loading. */
	const loading = $derived(artifactStore.navTreeLoading);

	/** Any error loading the tree. */
	const treeError = $derived(artifactStore.navTreeError);

	/**
	 * Return true when a path refers to a README file (excluded from artifact lists).
	 * @param path - The file path to test, which may be null.
	 * @returns True if the path ends with "README" or "README.md", false otherwise.
	 */
	function isReadmePath(path: string | null): boolean {
		if (!path) return false;
		const p = path.replace(/\\/g, "/");
		const name = p.split("/").pop() ?? "";
		return name === "README" || name === "README.md";
	}

	/** All nodes from the navTree, with README filtered out. */
	const rawNodes = $derived(allNodes.filter((n) => !isReadmePath(n.path)));

	/** Whether nodes form a tree (have children) or a flat list. */
	const isTree = $derived(rawNodes.some((n) => n.children !== null));

	// ---- View state (reactive, per category) ----

	let currentSort = $state<SortConfig>({ field: "title", direction: "asc" });
	let currentFilters = $state<Readonly<Record<string, readonly string[]>>>({});
	let currentGroup = $state<string | null>(null);

	// When category changes, load the view state for it
	$effect(() => {
		const state = getViewState(category);
		currentSort = state.sort;
		currentFilters = state.filters;
		currentGroup = state.group;
	});

	/**
	 * Persist the new sort config and update reactive sort state.
	 * @param sort - The new sort configuration containing field and direction.
	 */
	function handleSortChange(sort: SortConfig) {
		currentSort = sort;
		const state = getViewState(category);
		viewStates.set(category, { ...state, sort });
	}

	/**
	 * Persist the new filter state and update reactive filter state.
	 * @param filters - A map of field names to selected filter values.
	 */
	function handleFilterChange(filters: Record<string, readonly string[]>) {
		currentFilters = filters;
		const state = getViewState(category);
		viewStates.set(category, { ...state, filters });
	}

	/**
	 * Persist the new group-by field and update reactive group state.
	 * @param group - The field name to group artifacts by, or null to disable grouping.
	 */
	function handleGroupChange(group: string | null) {
		currentGroup = group;
		const state = getViewState(category);
		viewStates.set(category, { ...state, group });
	}

	// ---- Processed nodes (filter → sort) ----

	const processedNodes = $derived.by(() => {
		if (isTree) return rawNodes;
		const filtered = applyFilters(rawNodes, currentFilters);
		return applySort(filtered, currentSort);
	});

	/** Grouped sections, only used when currentGroup is set. */
	const groupedNodes = $derived.by(() => {
		if (isTree || !currentGroup) return null;
		return applyGrouping(
			processedNodes,
			currentGroup,
			currentNavType?.navigation_config?.defaults?.group_order?.[currentGroup],
			currentNavType?.filterable_fields ?? [],
		);
	});

	// ---- Breadcrumb helpers ----

	/**
	 * Convert a kebab-case path segment into a title-cased display label.
	 * @param segment - A single path segment such as "my-artifacts".
	 * @returns A human-readable label such as "My Artifacts".
	 */
	function humanizeSegment(segment: string): string {
		return segment
			.split("-")
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(" ");
	}

	/**
	 * Build the ordered breadcrumb label array for a node, including group, category, and folder hierarchy.
	 * @param node - The artifact tree node for which breadcrumbs are being generated.
	 * @returns An array of display label strings from the root group down to the artifact title.
	 */
	function buildBreadcrumbs(node: DocNode): string[] {
		const crumbs: string[] = [];

		// Add group label if in a group
		const group = navigationStore.activeGroup;
		if (group) {
			crumbs.push(navigationStore.getLabelForKey(group));
			// Only add type label if the group has multiple sub-categories.
			const groupChildren = navigationStore.getGroupChildren(group);
			if (groupChildren.length > 1) {
				crumbs.push(categoryLabel);
			}
		} else {
			crumbs.push(categoryLabel);
		}

		// Add folder hierarchy for tree items.
		if (isTree && node.path && currentNavType) {
			const typeRoot = currentNavType.path.replace(/\\/g, "/").replace(/\/$/, "");
			const normalizedPath = node.path.replace(/\\/g, "/");
			const relativePath = normalizedPath.startsWith(typeRoot + "/")
				? normalizedPath.slice(typeRoot.length + 1)
				: normalizedPath;
			const segments = relativePath.split("/");
			// All segments except the last are intermediate folders
			for (let i = 0; i < segments.length - 1; i++) {
				crumbs.push(humanizeSegment(segments[i]));
			}
		}

		// Add the item name
		crumbs.push(node.label);

		return crumbs;
	}

	/**
	 * Navigate to the artifact represented by the clicked leaf node.
	 * @param node - The leaf DocNode that the user clicked in the navigation tree.
	 */
	function handleLeafClick(node: DocNode) {
		if (!node.path) return;
		navigationStore.openArtifact(node.path, buildBreadcrumbs(node));
	}
</script>

<Stack gap={0} height="full">
	{#if !isTree}
		<ArtifactToolbar
			sortableFields={currentNavType?.sortable_fields ?? []}
			filterableFields={currentNavType?.filterable_fields ?? []}
			navigationConfig={currentNavType?.navigation_config}
			nodes={rawNodes}
			{currentSort}
			{currentFilters}
			{currentGroup}
			onSortChange={handleSortChange}
			onFilterChange={handleFilterChange}
			onGroupChange={handleGroupChange}
		/>
	{/if}

	<Box minHeight={0} flex={1}>
		<ScrollArea>
			<Panel padding="tight">
				{#if loading}
					<Panel padding="normal"
						><Center full>
							<LoadingSpinner />
						</Center></Panel
					>
				{:else if treeError}
					<Panel padding="normal">
						<ErrorDisplay message={treeError} onRetry={() => artifactStore.loadNavTree()} />
					</Panel>
				{:else if rawNodes.length === 0}
					<Panel padding="normal">
						<EmptyState
							icon="file-text"
							title="No {categoryLabel.toLowerCase()} yet"
							description="No {categoryLabel.toLowerCase()} files found in this project."
						/>
					</Panel>
				{:else if processedNodes.length === 0}
					<Panel padding="tight"
						><Center full>
							<Caption>No matching items.</Caption>
						</Center></Panel
					>
				{:else if isTree}
					<Stack gap={0}>
						{#each processedNodes as node (node.path ?? node.label)}
							{@render treeSection(node, 0)}
						{/each}
					</Stack>
				{:else if groupedNodes !== null}
					{@const collapsedDefaults =
						currentNavType?.navigation_config?.defaults?.collapsed_groups ?? []}
					<Stack gap={0}>
						{#each groupedNodes as group (group.label)}
							<Collapsible open={!collapsedDefaults.includes(group.label.toLowerCase())}>
								<CollapsibleGroupHeader label={group.label} count={group.nodes.length} />
								<CollapsibleContent>
									{#each group.nodes as node (node.path)}
										<ArtifactListItem
											label={node.label}
											description={node.description ?? undefined}
											status={node.status ?? undefined}
											path={node.path ?? undefined}
											onclick={() => handleLeafClick(node)}
										/>
									{/each}
								</CollapsibleContent>
							</Collapsible>
						{/each}
					</Stack>
				{:else}
					{#each processedNodes as node (node.path)}
						<ArtifactListItem
							label={node.label}
							description={node.description ?? undefined}
							status={node.status ?? undefined}
							path={node.path ?? undefined}
							onclick={() => handleLeafClick(node)}
						/>
					{/each}
				{/if}
			</Panel>
		</ScrollArea>
	</Box>
</Stack>

{#snippet treeSection(node: DocNode, depth: number)}
	{#if node.children}
		{@const dirIconName = resolveDirectoryIcon(node.icon)}
		<Collapsible open={true}>
			<TreeCollapsibleTrigger {depth}>
				<Icon name="chevron-right" size="xs" />
				<Icon name={dirIconName} size="xs" />
				{node.label}
			</TreeCollapsibleTrigger>
			<CollapsibleContent>
				{#each node.children as child (child.path ?? child.label)}
					{@render treeSection(child, depth + 1)}
				{/each}
			</CollapsibleContent>
		</Collapsible>
	{:else if node.path}
		<TreeIndent {depth}>
			<ArtifactListItem
				label={node.label}
				description={node.description ?? undefined}
				status={node.status ?? undefined}
				path={node.path ?? undefined}
				onclick={() => handleLeafClick(node)}
			/>
		</TreeIndent>
	{/if}
{/snippet}
