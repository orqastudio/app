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
		type IssueGroup,
	} from "../../stores/issue-store.svelte.js";
	import { events, type LogEvent } from "../../stores/log-store.svelte.js";
	import { openDrawer } from "../../stores/drawer-store.svelte.js";

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
	const ISSUE_LEVELS = new Set(["WARN", "ERROR"]);

	const filteredGroups = $derived(
		issueGroups
			.filter((g) => ISSUE_LEVELS.has(g.level))
			.filter(
				(g) =>
					searchQuery.trim().length === 0 ||
					g.title.toLowerCase().includes(searchQuery.trim().toLowerCase()),
			),
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

	/**
	 * Build a synthetic LogEvent from issue group metadata. Used when the group's
	 * most recent event is not present in the live event buffer (e.g. daemon
	 * restarted, buffer rolled over). The level and source fields are cast from
	 * the group's string values — the backend guarantees they match the enum.
	 * @param group - The issue group to synthesize an event from.
	 * @returns A LogEvent shaped from the group's metadata.
	 */
	function syntheticEvent(group: IssueGroup): LogEvent {
		return {
			id: group.recent_event_ids[0] ?? 0,
			timestamp: group.last_seen,
			level: group.level as LogEvent["level"],
			source: group.component as LogEvent["source"],
			category: "",
			message: group.title,
			metadata: null,
			session_id: null,
			fingerprint: group.fingerprint,
			message_template: group.title,
			correlation_id: undefined,
			stack_frames: [],
		};
	}

	/**
	 * Handle an issue row click. Selects the issue in the store and opens the
	 * EventDrawer. Prefers the matching event from the live buffer; when the
	 * event is not in the buffer (e.g. loaded from history or not yet streamed),
	 * builds a synthetic event from the group metadata so the drawer always opens.
	 * The navigation list is built from the most recent event of each visible group.
	 * @param group - The issue group that was clicked.
	 */
	function handleIssueClick(group: IssueGroup): void {
		selectIssue(group.fingerprint);

		// Prefer a real event from the live buffer; fall back to a synthetic one
		// built from group metadata so the click always opens the drawer.
		const recentId = group.recent_event_ids[0];
		const bufferedEvent =
			recentId !== undefined ? events.find((ev) => ev.id === recentId) : undefined;
		const event = bufferedEvent ?? syntheticEvent(group);

		// Build a navigation list from the most recent event of each visible group.
		// Groups without a matching event in the buffer use a synthetic event.
		const navList = filteredGroups.map((g) => {
			const id = g.recent_event_ids[0];
			const found = id !== undefined ? events.find((ev) => ev.id === id) : undefined;
			return found ?? syntheticEvent(g);
		});

		openDrawer(event, navList.length > 0 ? navList : [event]);
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
						onclick={() => handleIssueClick(group)}
					/>
				{/each}
			</Stack>
		</ScrollArea>
	{/if}
</Stack>
