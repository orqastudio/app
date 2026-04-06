<!-- IssueFilters — sort and filter controls bar for the Issues list.
Composes SectionHeader, HStack, SearchInput, SelectMenu, Button, and Icon
into a single toolbar. All state is lifted: parent owns sortBy, sortDir,
filterLevel, filterComponent, and searchQuery. -->
<script lang="ts">
	import SectionHeader from "../section-header/SectionHeader.svelte";
	import SearchInput from "../search-input/SearchInput.svelte";
	import SelectMenu from "../select-menu/SelectMenu.svelte";
	import Button from "../button/button.svelte";
	import Icon from "../icon/Icon.svelte";
	import { HStack } from "../layout/index.js";

	/** Available sort fields for the issues list. */
	type SortBy = "last_seen" | "count" | "level" | "component";
	/** Sort direction. */
	type SortDir = "asc" | "desc";

	let {
		sortBy = $bindable<SortBy>("last_seen"),
		sortDir = $bindable<SortDir>("desc"),
		filterLevel = $bindable<string | undefined>(undefined),
		filterComponent = $bindable<string | undefined>(undefined),
		searchQuery = $bindable<string>(""),
		components = [] as string[],
		onSortChange,
		onFilterLevelChange,
		onFilterComponentChange,
		onSearchChange,
	}: {
		/** Active sort field. */
		sortBy: SortBy;
		/** Active sort direction. */
		sortDir: SortDir;
		/** Active level filter value, or undefined for all levels. */
		filterLevel?: string;
		/** Active component filter value, or undefined for all components. */
		filterComponent?: string;
		/** Current search query string. */
		searchQuery?: string;
		/** Available component names for the component filter dropdown. */
		components?: string[];
		/** Called when sort field or direction changes. */
		onSortChange?: (sortBy: string, sortDir: string) => void;
		/** Called when the level filter changes; undefined means all levels. */
		onFilterLevelChange?: (level: string | undefined) => void;
		/** Called when the component filter changes; undefined means all components. */
		onFilterComponentChange?: (component: string | undefined) => void;
		/** Called when the search query changes. */
		onSearchChange?: (query: string) => void;
	} = $props();

	/** Static sort field options shown in the sort SelectMenu. */
	const sortOptions = [
		{ value: "last_seen", label: "Last seen" },
		{ value: "count", label: "Count" },
		{ value: "level", label: "Severity" },
		{ value: "component", label: "Component" },
	];

	/** Static level filter options; empty string represents "all levels". */
	const levelOptions = [
		{ value: "", label: "All levels" },
		{ value: "Error", label: "Error" },
		{ value: "Warn", label: "Warning" },
		{ value: "Info", label: "Info" },
		{ value: "Debug", label: "Debug" },
	];

	/** Component filter options built from the components prop plus an "all" entry. */
	const componentOptions = $derived([
		{ value: "", label: "All components" },
		...components.map((c) => ({ value: c, label: c })),
	]);

	/** Derive the trigger label for the sort SelectMenu from the active sortBy value. */
	const sortTriggerLabel = $derived(sortOptions.find((o) => o.value === sortBy)?.label ?? "Sort");

	/** Derive the trigger label for the level SelectMenu from the active filterLevel value. */
	const levelTriggerLabel = $derived(
		levelOptions.find((o) => o.value === (filterLevel ?? ""))?.label ?? "Level",
	);

	/** Derive the trigger label for the component SelectMenu from the active filterComponent value. */
	const componentTriggerLabel = $derived(
		componentOptions.find((o) => o.value === (filterComponent ?? ""))?.label ?? "Component",
	);

	/** Local mutable string for SearchInput two-way binding. Syncs back to searchQuery prop. */
	let localSearch = $state(searchQuery ?? "");

	/** Propagate local search changes back to the bindable prop and parent callback. */
	$effect(() => {
		searchQuery = localSearch;
		onSearchChange?.(localSearch);
	});

	/**
	 * Handle sort field selection from the SelectMenu.
	 * Updates sortBy and notifies parent via onSortChange.
	 * @param value - The selected sort column key.
	 */
	function handleSortSelect(value: string): void {
		sortBy = value as SortBy;
		onSortChange?.(sortBy, sortDir);
	}

	/**
	 * Toggle between ascending and descending sort direction.
	 * Notifies parent via onSortChange after toggling.
	 */
	function toggleSortDir(): void {
		sortDir = sortDir === "asc" ? "desc" : "asc";
		onSortChange?.(sortBy, sortDir);
	}

	/**
	 * Handle level filter selection from the SelectMenu.
	 * Empty string maps back to undefined (all levels).
	 * @param value - The selected level key, or empty string for "all".
	 */
	function handleLevelSelect(value: string): void {
		const resolved: string | undefined = value === "" ? undefined : value;
		filterLevel = resolved;
		onFilterLevelChange?.(resolved);
	}

	/**
	 * Handle component filter selection from the SelectMenu.
	 * Empty string maps back to undefined (all components).
	 * @param value - The selected component key, or empty string for "all".
	 */
	function handleComponentSelect(value: string): void {
		const resolved: string | undefined = value === "" ? undefined : value;
		filterComponent = resolved;
		onFilterComponentChange?.(resolved);
	}
</script>

<SectionHeader variant="section">
	{#snippet start()}
		<HStack gap={2}>
			<SearchInput placeholder="Filter issues..." bind:value={localSearch} />
			<SelectMenu
				items={levelOptions}
				selected={filterLevel ?? ""}
				triggerLabel={levelTriggerLabel}
				onSelect={handleLevelSelect}
			/>
			{#if components.length > 0}
				<SelectMenu
					items={componentOptions}
					selected={filterComponent ?? ""}
					triggerLabel={componentTriggerLabel}
					onSelect={handleComponentSelect}
				/>
			{/if}
		</HStack>
	{/snippet}
	{#snippet end()}
		<HStack gap={1}>
			<SelectMenu
				items={sortOptions}
				selected={sortBy}
				triggerLabel={sortTriggerLabel}
				onSelect={handleSortSelect}
			/>
			<Button variant="ghost" size="icon-sm" onclick={toggleSortDir}>
				<Icon name={sortDir === "asc" ? "arrow-up" : "arrow-down"} size="sm" />
			</Button>
		</HStack>
	{/snippet}
</SectionHeader>
