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
		Caption,
		PopoverRoot,
		PopoverTrigger,
		PopoverContent,
	} from "@orqastudio/svelte-components/pure";
	import { SvelteSet } from "svelte/reactivity";
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
     Scoped CSS class provides layout; no Tailwind utility classes used. -->
<div class="log-filters" role="toolbar" aria-label="Log filters">
	<!-- Source multi-select dropdown: PopoverRoot/Trigger/Content replace the raw dropdown divs. -->
	<PopoverRoot bind:open={sourceOpen}>
		<PopoverTrigger>
			<!-- Wrapper span with display:contents provides :global() hook for Button overrides. -->
			<span class="log-filters__trigger-wrap" style="display: contents;">
				<Button variant="outline" size="sm" aria-haspopup="listbox" aria-expanded={sourceOpen}>
					Source
					{#if filters.sources.size > 0}
						<!-- Wrapper span for Badge count indicator. -->
						<span class="log-filters__count-wrap" style="display: contents;">
							<Badge variant="default">
								{filters.sources.size}
							</Badge>
						</span>
					{/if}
					<!-- Chevron character: rendered inline. -->
					{sourceOpen ? "▲" : "▼"}
				</Button>
			</span>
		</PopoverTrigger>
		<PopoverContent align="start">
			<div
				class="log-filters__dropdown-panel"
				role="listbox"
				aria-multiselectable="true"
				aria-label="Source filter"
			>
				{#each ALL_SOURCES as source (source)}
					<!-- Wrapper span provides :global() hook for option Button overrides. -->
					<span class="log-filters__option-wrap" style="display: contents;">
						<Button
							variant="ghost"
							role="option"
							aria-selected={filters.sources.has(source)}
							onclick={() => toggleSource(source)}
						>
							<!-- Checkbox component provides the visual indicator; pointer-events none
							     via scoped CSS lets the Button's onclick handle the toggle. -->
							<span class="log-filters__checkbox-wrap">
								<Checkbox checked={filters.sources.has(source)} aria-hidden="true" tabindex={-1} />
							</span>
							<Caption>{source}</Caption>
						</Button>
					</span>
				{/each}
			</div>
		</PopoverContent>
	</PopoverRoot>

	<!-- Level checkbox group (inline, no dropdown needed — only 5 values).
	     HStack provides flex-row layout; scoped CSS sets gap/shrink. -->
	<HStack gap={1} role="group" aria-label="Level filter">
		{#each ALL_LEVELS as level (level)}
			<!-- Label accepts onclick via restProps for the toggle action.
			     Wrapper span provides :global() hook for Badge overrides. -->
			<Label onclick={() => toggleLevel(level)}>
				<Checkbox checked={filters.levels.has(level)} aria-label={level} />
				<span class="log-filters__level-badge-wrap" style="display: contents;">
					<Badge variant={LEVEL_BADGE_VARIANT[level] ?? "outline"}>
						{level}
					</Badge>
				</span>
			</Label>
		{/each}
	</HStack>

	<!-- Category multi-select dropdown: PopoverRoot/Trigger/Content replace the raw dropdown divs. -->
	<PopoverRoot bind:open={categoryOpen}>
		<PopoverTrigger>
			<span class="log-filters__trigger-wrap" style="display: contents;">
				<Button
					variant="outline"
					size="sm"
					aria-haspopup="listbox"
					aria-expanded={categoryOpen}
					disabled={knownCategories.size === 0}
				>
					Category
					{#if filters.categories.size > 0}
						<span class="log-filters__count-wrap" style="display: contents;">
							<Badge variant="default">
								{filters.categories.size}
							</Badge>
						</span>
					{/if}
					<!-- Chevron character: rendered inline. -->
					{categoryOpen ? "▲" : "▼"}
				</Button>
			</span>
		</PopoverTrigger>
		{#if knownCategories.size > 0}
			<PopoverContent align="start">
				<div
					class="log-filters__dropdown-panel log-filters__dropdown-panel--scrollable"
					role="listbox"
					aria-multiselectable="true"
					aria-label="Category filter"
				>
					{#each [...knownCategories].sort() as category (category)}
						<span class="log-filters__option-wrap" style="display: contents;">
							<Button
								variant="ghost"
								role="option"
								aria-selected={filters.categories.has(category)}
								onclick={() => toggleCategory(category)}
							>
								<!-- Checkbox provides visual indicator; pointer-events none via scoped CSS
								     lets the Button's onclick handle the toggle. -->
								<span class="log-filters__checkbox-wrap">
									<Checkbox
										checked={filters.categories.has(category)}
										aria-hidden="true"
										tabindex={-1}
									/>
								</span>
								<Caption truncate>{category}</Caption>
							</Button>
						</span>
					{/each}
				</div>
			</PopoverContent>
		{/if}
	</PopoverRoot>

	<!-- Full-text search input. The id allows the global Ctrl+F handler in
	     +layout.svelte to focus this element from outside this component. -->
	<span class="log-filters__search-wrap">
		<Input
			id="log-search-input"
			type="search"
			placeholder="Search messages…"
			bind:value={filters.searchText}
			aria-label="Search log messages"
		/>
	</span>

	<!-- Clear all filters button — only visible when a filter is active. -->
	{#if hasActiveFilters}
		<span class="log-filters__clear-wrap" style="display: contents;">
			<Button variant="outline" size="sm" onclick={clearFilters} aria-label="Clear all filters">
				Clear filters
			</Button>
		</span>
	{/if}
</div>

<style>
	/* Filter bar: compact strip above the log table. Uses flex-row layout
	   with wrapping; HStack is not used here because the root needs role=toolbar. */
	.log-filters {
		display: flex;
		align-items: center;
		gap: var(--spacing-2);
		padding: 0 var(--spacing-2);
		border-bottom: 1px solid var(--color-border);
		background-color: var(--color-surface-base);
		min-height: 32px;
		flex-shrink: 0;
		flex-wrap: wrap;
	}

	/* Trigger button override: compact height and font for filter bar fit.
	   Targets Button inside the trigger wrapper span. */
	:global(.log-filters__trigger-wrap button) {
		font-size: 11px !important;
		height: 24px !important;
		padding: 0 var(--spacing-2) !important;
		gap: var(--spacing-1) !important;
	}

	/* Count badge inside dropdown trigger: extra compact.
	   Targets Badge inside the count wrapper span. */
	:global(.log-filters__count-wrap [data-slot="badge"]) {
		font-size: 10px !important;
		padding: 0 var(--spacing-1) !important;
		line-height: 1 !important;
	}

	/* Dropdown panel content: inner div wrapping the listbox options. */
	.log-filters__dropdown-panel {
		display: flex;
		flex-direction: column;
		min-width: 130px;
	}

	/* Scrollable variant for category dropdown. */
	.log-filters__dropdown-panel--scrollable {
		max-height: 12rem;
		min-width: 160px;
		overflow-y: auto;
	}

	/* Individual option row inside a dropdown — overrides Button defaults for compact list fit.
	   Targets Button inside each option wrapper span. */
	:global(.log-filters__option-wrap button) {
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

	/* Checkbox inside dropdown options: compact, non-interactive (click handled by Button).
	   Wrapper span keeps the checkbox contained; pointer-events none prevents interference. */
	.log-filters__checkbox-wrap {
		flex-shrink: 0;
		display: flex;
		align-items: center;
		pointer-events: none;
	}

	:global(.log-filters__checkbox-wrap [data-slot="checkbox"]) {
		width: 12px !important;
		height: 12px !important;
	}

	/* Level label: clickable row with flex alignment.
	   Targets the <label> elements rendered by Label inside the filter bar. */
	:global(.log-filters label) {
		display: flex !important;
		cursor: pointer !important;
		align-items: center !important;
		gap: var(--spacing-1) !important;
		font-size: inherit !important;
		font-weight: inherit !important;
	}

	/* Level badge: compact variant.
	   Targets Badge inside the level badge wrapper span. */
	:global(.log-filters__level-badge-wrap [data-slot="badge"]) {
		font-size: 10px !important;
		padding: 1px var(--spacing-1) !important;
		line-height: 1 !important;
	}

	/* Search input wrapper: stretches to fill remaining space. */
	.log-filters__search-wrap {
		flex: 1;
		min-width: 180px;
		display: flex;
	}

	/* Search input: compact height for filter bar. */
	:global(.log-filters__search-wrap input) {
		height: 24px !important;
		font-size: 11px !important;
		padding: 0 var(--spacing-2) !important;
	}

	/* Clear filters button: danger-accent hover state.
	   Targets Button inside the clear wrapper span. */
	:global(.log-filters__clear-wrap button) {
		font-size: 11px !important;
		height: 24px !important;
		padding: 0 var(--spacing-2) !important;
	}

	:global(.log-filters__clear-wrap button:hover) {
		border-color: color-mix(in srgb, var(--color-destructive) 40%, transparent) !important;
		background-color: color-mix(in srgb, var(--color-destructive) 10%, transparent) !important;
		color: var(--color-destructive) !important;
	}
</style>
