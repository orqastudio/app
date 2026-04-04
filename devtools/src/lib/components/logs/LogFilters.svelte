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
	} from "@orqastudio/svelte-components/pure";
	import {
		filters,
		hasActiveFilters as getHasActiveFilters,
		knownCategories as getKnownCategories,
		clearFilters,
		ALL_LEVELS,
		ALL_SOURCES,
		type LogEvent,
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
	function toggleSet<T>(set: Set<T>, value: T): Set<T> {
		const next = new Set(set);
		if (next.has(value)) {
			next.delete(value);
		} else {
			next.add(value);
		}
		return next;
	}

	// Toggle a source in the active source filter set.
	function toggleSource(source: LogEvent["source"]): void {
		filters.sources = toggleSet(filters.sources, source);
	}

	// Toggle a level in the active level filter set.
	function toggleLevel(level: LogEvent["level"]): void {
		filters.levels = toggleSet(filters.levels, level);
	}

	// Toggle a category in the active category filter set.
	function toggleCategory(category: string): void {
		filters.categories = toggleSet(filters.categories, category);
	}

	// Close dropdowns when the user clicks outside them.
	function handleDocumentClick(e: MouseEvent): void {
		const target = e.target as HTMLElement;
		if (!target.closest("[data-source-dropdown]")) sourceOpen = false;
		if (!target.closest("[data-category-dropdown]")) categoryOpen = false;
	}

	// Badge variant for the level indicators in the checkbox group.
	// Matches the variants used in LogRow so the filter labels mirror the table.
	const LEVEL_BADGE_VARIANT: Record<string, "secondary" | "destructive" | "outline" | "default" | "warning"> = {
		Debug: "outline",
		Info: "default",
		Warn: "warning",
		Error: "destructive",
		Perf: "secondary",
	};
</script>

<svelte:document onclick={handleDocumentClick} />

<!-- Filter bar: compact single-line strip above the log table. -->
<div
	class="log-filters"
	style="min-height: 32px;"
	role="toolbar"
	aria-label="Log filters"
>
	<!-- Source multi-select dropdown -->
	<div class="log-filters__dropdown" data-source-dropdown>
		<Button
			variant="outline"
			size="sm"
			class="log-filters__dropdown-trigger"
			onclick={() => { sourceOpen = !sourceOpen; categoryOpen = false; }}
			aria-haspopup="listbox"
			aria-expanded={sourceOpen}
		>
			Source
			{#if filters.sources.size > 0}
				<Badge variant="default" class="log-filters__count-badge">
					{filters.sources.size}
				</Badge>
			{/if}
			<span class="log-filters__chevron">{sourceOpen ? "▲" : "▼"}</span>
		</Button>

		{#if sourceOpen}
			<div
				class="log-filters__dropdown-panel"
				role="listbox"
				aria-multiselectable="true"
				aria-label="Source filter"
			>
				{#each ALL_SOURCES as source (source)}
					<Button
						variant="ghost"
						class="log-filters__option"
						role="option"
						aria-selected={filters.sources.has(source)}
						onclick={() => toggleSource(source)}
					>
						<span
							class="log-filters__checkbox {filters.sources.has(source) ? 'log-filters__checkbox--checked' : ''}"
						></span>
						<span class="log-filters__option-label">{source}</span>
					</Button>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Level checkbox group (inline, no dropdown needed — only 5 values) -->
	<div class="log-filters__level-group" role="group" aria-label="Level filter">
		{#each ALL_LEVELS as level (level)}
			<Label class="log-filters__level-label">
				<Checkbox
					checked={filters.levels.has(level)}
					onCheckedChange={() => toggleLevel(level)}
					class="log-filters__level-checkbox"
					aria-label={level}
				/>
				<Badge
					variant={LEVEL_BADGE_VARIANT[level] ?? "outline"}
					class="log-filters__level-badge"
				>
					{level}
				</Badge>
			</Label>
		{/each}
	</div>

	<!-- Category multi-select dropdown (populated from knownCategories) -->
	<div class="log-filters__dropdown" data-category-dropdown>
		<Button
			variant="outline"
			size="sm"
			class="log-filters__dropdown-trigger"
			onclick={() => { categoryOpen = !categoryOpen; sourceOpen = false; }}
			aria-haspopup="listbox"
			aria-expanded={categoryOpen}
			disabled={knownCategories.size === 0}
		>
			Category
			{#if filters.categories.size > 0}
				<Badge variant="default" class="log-filters__count-badge">
					{filters.categories.size}
				</Badge>
			{/if}
			<span class="log-filters__chevron">{categoryOpen ? "▲" : "▼"}</span>
		</Button>

		{#if categoryOpen && knownCategories.size > 0}
			<div
				class="log-filters__dropdown-panel log-filters__dropdown-panel--scrollable"
				role="listbox"
				aria-multiselectable="true"
				aria-label="Category filter"
			>
				{#each [...knownCategories].sort() as category (category)}
					<Button
						variant="ghost"
						class="log-filters__option"
						role="option"
						aria-selected={filters.categories.has(category)}
						onclick={() => toggleCategory(category)}
					>
						<span
							class="log-filters__checkbox {filters.categories.has(category) ? 'log-filters__checkbox--checked' : ''}"
						></span>
						<span class="log-filters__option-label log-filters__option-label--truncate">{category}</span>
					</Button>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Full-text search input. The id allows the global Ctrl+F handler in
	     +layout.svelte to focus this element from outside this component. -->
	<Input
		id="log-search-input"
		type="search"
		class="log-filters__search"
		placeholder="Search messages…"
		bind:value={filters.searchText}
		aria-label="Search log messages"
	/>

	<!-- Clear all filters button — only visible when a filter is active -->
	{#if hasActiveFilters}
		<Button
			variant="outline"
			size="sm"
			class="log-filters__clear"
			onclick={clearFilters}
			aria-label="Clear all filters"
		>
			Clear filters
		</Button>
	{/if}
</div>

<style>
	/* Filter bar: compact strip above the log table. */
	.log-filters {
		border-bottom: 1px solid var(--color-border);
		background-color: var(--color-surface-base);
		display: flex;
		flex-shrink: 0;
		flex-wrap: wrap;
		align-items: center;
		gap: var(--spacing-2);
		padding: var(--spacing-1) var(--spacing-2);
	}

	/* Dropdown container: relative for the absolute panel. */
	.log-filters__dropdown {
		position: relative;
	}

	/* Override Button size for compact filter bar fit. */
	:global(.log-filters__dropdown-trigger) {
		font-size: 11px !important;
		height: 24px !important;
		padding: 0 var(--spacing-2) !important;
		gap: var(--spacing-1) !important;
	}

	/* Count badge inside dropdown trigger: extra compact. */
	:global(.log-filters__count-badge) {
		font-size: 10px !important;
		padding: 0 var(--spacing-1) !important;
		line-height: 1 !important;
	}

	/* Chevron arrow in dropdown trigger. */
	.log-filters__chevron {
		font-size: 9px;
		color: var(--color-content-muted);
	}

	/* Dropdown panel: floats below the trigger. */
	.log-filters__dropdown-panel {
		position: absolute;
		left: 0;
		top: 100%;
		z-index: 10;
		margin-top: var(--spacing-1);
		min-width: 130px;
		border-radius: var(--radius-md);
		border: 1px solid var(--color-border);
		background-color: var(--color-surface-raised);
		box-shadow: var(--shadow-lg);
	}

	/* Scrollable variant for category dropdown. */
	.log-filters__dropdown-panel--scrollable {
		max-height: 12rem;
		min-width: 160px;
		overflow-y: auto;
	}

	/* Individual option row inside a dropdown — overrides Button defaults for compact list fit. */
	:global(.log-filters__option) {
		display: flex !important;
		width: 100% !important;
		justify-content: flex-start !important;
		align-items: center !important;
		gap: var(--spacing-2) !important;
		padding: var(--spacing-1) var(--spacing-2) !important;
		text-align: left !important;
		font-size: 11px !important;
		height: auto !important;
		border-radius: 0 !important;
	}

	/* Custom checkbox square. */
	.log-filters__checkbox {
		width: 12px;
		height: 12px;
		flex-shrink: 0;
		border-radius: var(--radius-sm);
		border: 1px solid var(--color-border);
		background-color: transparent;
	}

	.log-filters__checkbox--checked {
		background-color: var(--color-accent-base);
		border-color: var(--color-accent-base);
	}

	/* Option label text. */
	.log-filters__option-label {
		color: var(--color-content-base);
	}

	.log-filters__option-label--truncate {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	/* Level group: inline row of checkbox + badge pairs. */
	.log-filters__level-group {
		display: flex;
		align-items: center;
		gap: var(--spacing-1-5);
	}

	:global(.log-filters__level-label) {
		display: flex !important;
		cursor: pointer !important;
		align-items: center !important;
		gap: var(--spacing-1) !important;
		font-size: inherit !important;
		font-weight: inherit !important;
	}

	/* Level badge label: compact variant. */
	:global(.log-filters__level-badge) {
		font-size: 10px !important;
		padding: 1px var(--spacing-1) !important;
		line-height: 1 !important;
	}

	/* Search input: stretches to fill remaining space, compact height for filter bar. */
	:global(.log-filters__search) {
		flex: 1 !important;
		min-width: 180px !important;
		height: 24px !important;
		font-size: 11px !important;
		padding: 0 var(--spacing-2) !important;
	}

	/* Clear filters: danger-accent hover state. */
	:global(.log-filters__clear) {
		font-size: 11px !important;
		height: 24px !important;
		padding: 0 var(--spacing-2) !important;
	}

	:global(.log-filters__clear:hover) {
		border-color: color-mix(in srgb, var(--color-destructive) 40%, transparent) !important;
		background-color: color-mix(in srgb, var(--color-destructive) 10%, transparent) !important;
		color: var(--color-destructive) !important;
	}
</style>
