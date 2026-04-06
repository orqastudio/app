<!-- IssuesView — wiring layer connecting the issue store to library display components.
     Initialises the issue store on mount, renders IssueFilters at the top, and renders
     one IssueRow per issue group inside a ScrollArea. Shows EmptyState when the list
     is empty. Client-side text filtering is applied when searchQuery is non-empty. -->
<script lang="ts">
	import { onMount } from "svelte";
	import {
		Stack,
		Center,
		ScrollArea,
		EmptyState,
		IssueRow,
		IssueFilters,
	} from "@orqastudio/svelte-components/pure";
	import {
		init,
		issueGroups,
		sortBy,
		sortDir,
		filterComponent,
		filterLevel,
		selectedFingerprint,
		selectIssue,
		loadIssueGroups,
		type IssueSortBy,
		type SortDir,
	} from "../../stores/issue-store.svelte.js";

	/** Current text search query — client-side only, not sent to backend. */
	let searchQuery = $state("");

	/**
	 * Derive unique component names from the loaded issue groups for the
	 * component filter dropdown in IssueFilters.
	 * @returns Sorted, deduplicated list of component name strings.
	 */
	const availableComponents = $derived([...new Set(issueGroups.map((g) => g.component))].sort());

	/**
	 * Apply client-side text filtering on top of the store-filtered groups.
	 * Matches against issue title (case-insensitive). When searchQuery is empty
	 * the full issueGroups list is returned unchanged.
	 * @returns Issue groups matching the current searchQuery.
	 */
	const filteredGroups = $derived(
		searchQuery.trim().length === 0
			? issueGroups
			: issueGroups.filter((g) => g.title.toLowerCase().includes(searchQuery.trim().toLowerCase())),
	);

	/**
	 * Handle sort field or direction change from IssueFilters. Updates the store
	 * state and triggers a backend reload.
	 * @param newSortBy - The selected sort column.
	 * @param newSortDir - The selected sort direction.
	 */
	function handleSortChange(newSortBy: string, newSortDir: string): void {
		sortBy.value = newSortBy as IssueSortBy;
		sortDir.value = newSortDir as SortDir;
		loadIssueGroups();
	}

	/**
	 * Handle severity level filter change from IssueFilters. Updates the store
	 * and triggers a backend reload. Undefined clears the filter.
	 * @param level - The selected level string, or undefined for all levels.
	 */
	function handleFilterLevel(level: string | undefined): void {
		filterLevel.value = level ?? "";
		loadIssueGroups();
	}

	/**
	 * Handle component filter change from IssueFilters. Updates the store and
	 * triggers a backend reload. Undefined clears the filter.
	 * @param component - The selected component string, or undefined for all.
	 */
	function handleFilterComponent(component: string | undefined): void {
		filterComponent.value = component ?? "";
		loadIssueGroups();
	}

	/**
	 * Handle search query changes from IssueFilters. Applies client-side
	 * filtering only — no backend reload triggered.
	 * @param query - The current search query string.
	 */
	function handleSearch(query: string): void {
		searchQuery = query;
	}

	onMount(() => {
		let cleanup: (() => void) | undefined;

		init().then((fn) => {
			cleanup = fn;
		});

		return () => {
			cleanup?.();
		};
	});
</script>

<Stack gap={0} height="full">
	<IssueFilters
		sortBy={sortBy.value}
		sortDir={sortDir.value}
		filterLevel={filterLevel.value === "" ? undefined : filterLevel.value}
		filterComponent={filterComponent.value === "" ? undefined : filterComponent.value}
		{searchQuery}
		components={availableComponents}
		onSortChange={handleSortChange}
		onFilterLevelChange={handleFilterLevel}
		onFilterComponentChange={handleFilterComponent}
		onSearchChange={handleSearch}
	/>

	{#if filteredGroups.length === 0}
		<Center full>
			<EmptyState title="No issues" description="Events will appear here as they are captured." />
		</Center>
	{:else}
		<ScrollArea full>
			<Stack gap={0}>
				{#each filteredGroups as group (group.fingerprint)}
					<IssueRow
						{...group}
						selected={selectedFingerprint.value === group.fingerprint}
						onclick={() => selectIssue(group.fingerprint)}
					/>
				{/each}
			</Stack>
		</ScrollArea>
	{/if}
</Stack>
