<!-- Horizontal filter bar rendered above the log table. Provides source
     multi-select, level checkboxes, category multi-select, full-text search,
     and a Clear all button. Reads and writes the shared filter state in
     log-store.svelte.ts so LogTable reacts automatically. -->
<script lang="ts">
	import {
		Button,
		Badge,
		Input,
		Checkbox,
		Label,
		HStack,
		Box,
		Caption,
		PopoverRoot,
		PopoverTrigger,
		PopoverContent,
		Stack,
	} from "@orqastudio/svelte-components/pure";
	import { SvelteSet } from "svelte/reactivity";
	import {
		filters,
		hasActiveFilters as getHasActiveFilters,
		knownCategories as getKnownCategories,
		clearFilters,
		ALL_LEVELS,
		ALL_SOURCES,
		ALL_TIERS,
		type LogEvent,
		type EventTier,
	} from "../../stores/log-store.svelte.js";

	const hasActiveFilters = $derived(getHasActiveFilters());
	const knownCategories = $derived(getKnownCategories());

	// Whether the source dropdown is open.
	let sourceOpen = $state(false);

	// Whether the category dropdown is open.
	let categoryOpen = $state(false);

	// Toggle a value in a Set stored on the filters object. Svelte 5 $state
	// tracks object identity; we must reassign to trigger reactivity when the
	// Set contents change.
	/**
	 * Toggle a value in a reactive set, returning a new SvelteSet with the value added or removed.
	 * @param set - The existing set to base the new set on.
	 * @param value - The value to add if absent or remove if present.
	 * @returns A new SvelteSet with the toggled membership.
	 */
	function toggleSet<T>(set: Set<T>, value: T): SvelteSet<T> {
		const next = new SvelteSet(set);
		if (next.has(value)) {
			next.delete(value);
		} else {
			next.add(value);
		}
		return next;
	}

	/**
	 * Toggle a source in the active source filter set.
	 * @param source - The log source to add or remove from the filter.
	 */
	function toggleSource(source: LogEvent["source"]): void {
		filters.sources = toggleSet(filters.sources, source);
	}

	/**
	 * Toggle a level in the active level filter set.
	 * @param level - The log level to add or remove from the filter.
	 */
	function toggleLevel(level: LogEvent["level"]): void {
		filters.levels = toggleSet(filters.levels, level);
	}

	/**
	 * Toggle a tier in the active tier filter set.
	 * @param tier - The event tier to add or remove from the filter.
	 */
	function toggleTier(tier: EventTier): void {
		filters.tiers = toggleSet(filters.tiers, tier);
	}

	/**
	 * Toggle a category in the active category filter set.
	 * @param category - The category string to add or remove from the filter.
	 */
	function toggleCategory(category: string): void {
		filters.categories = toggleSet(filters.categories, category);
	}

	// Badge variant for the level indicators in the checkbox group.
	// Matches the variants used in LogRow so the filter labels mirror the table.
	const LEVEL_BADGE_VARIANT: Record<
		string,
		"secondary" | "destructive" | "outline" | "default" | "warning"
	> = {
		Debug: "outline",
		Info: "default",
		Warn: "warning",
		Error: "destructive",
		Perf: "secondary",
	};
</script>

<!-- Filter bar: compact single-line strip above the log table.
     HStack with role=toolbar provides the accessible landmark. -->
<HStack gap={2} wrap={true} role="toolbar" aria-label="Log filters">
	<!-- Source multi-select dropdown. -->
	<PopoverRoot bind:open={sourceOpen}>
		<PopoverTrigger>
			<Button variant="outline" size="xs" aria-haspopup="listbox" aria-expanded={sourceOpen}>
				Source
				{#if filters.sources.size > 0}
					<Badge variant="default" size="xs">
						{filters.sources.size}
					</Badge>
				{/if}
				{sourceOpen ? "▲" : "▼"}
			</Button>
		</PopoverTrigger>
		<PopoverContent align="start">
			<Stack gap={0} role="listbox" aria-multiselectable="true" aria-label="Source filter">
				{#each ALL_SOURCES as source (source)}
					<Button
						variant="ghost"
						full
						role="option"
						aria-selected={filters.sources.has(source)}
						onclick={() => toggleSource(source)}
					>
						<!-- Checkbox is aria-hidden; the Button onclick handles the toggle. -->
						<Checkbox checked={filters.sources.has(source)} aria-hidden="true" tabindex={-1} />
						<Caption>{source}</Caption>
					</Button>
				{/each}
			</Stack>
		</PopoverContent>
	</PopoverRoot>

	<!-- Tier toggle (Build / Runtime — only 2 values). -->
	<HStack gap={1} role="group" aria-label="Tier filter">
		{#each ALL_TIERS as tier (tier)}
			<Label onclick={() => toggleTier(tier)}>
				<Checkbox checked={filters.tiers.has(tier)} aria-label={tier} />
				<Badge variant={tier === "Build" ? "secondary" : "outline"} size="xs">
					{tier}
				</Badge>
			</Label>
		{/each}
	</HStack>

	<!-- Level checkbox group (inline, no dropdown needed — only 5 values). -->
	<HStack gap={1} role="group" aria-label="Level filter">
		{#each ALL_LEVELS as level (level)}
			<Label onclick={() => toggleLevel(level)}>
				<Checkbox checked={filters.levels.has(level)} aria-label={level} />
				<Badge variant={LEVEL_BADGE_VARIANT[level] ?? "outline"} size="xs">
					{level}
				</Badge>
			</Label>
		{/each}
	</HStack>

	<!-- Category multi-select dropdown. -->
	<PopoverRoot bind:open={categoryOpen}>
		<PopoverTrigger>
			<Button
				variant="outline"
				size="xs"
				aria-haspopup="listbox"
				aria-expanded={categoryOpen}
				disabled={knownCategories.size === 0}
			>
				Category
				{#if filters.categories.size > 0}
					<Badge variant="default" size="xs">
						{filters.categories.size}
					</Badge>
				{/if}
				{categoryOpen ? "▲" : "▼"}
			</Button>
		</PopoverTrigger>
		{#if knownCategories.size > 0}
			<PopoverContent align="start">
				<Stack gap={0} role="listbox" aria-multiselectable="true" aria-label="Category filter">
					{#each [...knownCategories].sort() as category (category)}
						<Button
							variant="ghost"
							full
							role="option"
							aria-selected={filters.categories.has(category)}
							onclick={() => toggleCategory(category)}
						>
							<!-- Checkbox is aria-hidden; the Button onclick handles the toggle. -->
							<Checkbox
								checked={filters.categories.has(category)}
								aria-hidden="true"
								tabindex={-1}
							/>
							<Caption truncate>{category}</Caption>
						</Button>
					{/each}
				</Stack>
			</PopoverContent>
		{/if}
	</PopoverRoot>

	<!-- Full-text search input. The id allows the global Ctrl+F handler in
	     +layout.svelte to focus this element from outside this component. -->
	<Box flex={1} minWidth={0}>
		<Input
			id="log-search-input"
			type="search"
			placeholder="Search messages…"
			bind:value={filters.searchText}
			aria-label="Search log messages"
		/>
	</Box>

	<!-- Clear all filters button — only visible when a filter is active. -->
	{#if hasActiveFilters}
		<Button variant="outline" size="xs" onclick={clearFilters} aria-label="Clear all filters">
			Clear filters
		</Button>
	{/if}
</HStack>
