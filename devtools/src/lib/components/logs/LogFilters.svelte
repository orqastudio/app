<!-- Horizontal filter bar rendered above the log table. Provides source
     multi-select, level checkboxes, category multi-select, full-text search,
     and a Clear all button. Reads and writes the shared filter state in
     log-store.svelte.ts so LogTable reacts automatically. -->
<script lang="ts">
	import {
		filters,
		hasActiveFilters,
		knownCategories,
		clearFilters,
		ALL_LEVELS,
		ALL_SOURCES,
		type LogEvent,
	} from "../../stores/log-store.svelte.js";

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

	// CSS badge colours for level checkboxes, matching LogRow badges.
	const LEVEL_COLOR: Record<string, string> = {
		Debug: "text-content-muted",
		Info: "text-blue-400",
		Warn: "text-yellow-400",
		Error: "text-red-400",
		Perf: "text-indigo-400",
	};
</script>

<svelte:document onclick={handleDocumentClick} />

<!-- Filter bar: compact single-line strip above the log table. -->
<div
	class="border-b border-border bg-surface-base flex shrink-0 flex-wrap items-center gap-2 px-2 py-1"
	style="min-height: 32px;"
	role="toolbar"
	aria-label="Log filters"
>
	<!-- Source multi-select dropdown -->
	<div class="relative" data-source-dropdown>
		<button
			class="flex items-center gap-1 rounded border border-border bg-surface-raised px-2 py-0.5 text-[11px] text-content-base transition-colors hover:border-border/80 hover:bg-surface-raised/80"
			onclick={() => { sourceOpen = !sourceOpen; categoryOpen = false; }}
			aria-haspopup="listbox"
			aria-expanded={sourceOpen}
		>
			Source
			{#if filters.sources.size > 0}
				<span class="rounded bg-blue-500/20 px-1 text-[10px] text-blue-400">{filters.sources.size}</span>
			{/if}
			<span class="text-content-muted text-[9px]">{sourceOpen ? "▲" : "▼"}</span>
		</button>

		{#if sourceOpen}
			<div
				class="absolute left-0 top-full z-10 mt-1 min-w-[130px] rounded border border-border bg-surface-raised shadow-lg"
				role="listbox"
				aria-multiselectable="true"
				aria-label="Source filter"
			>
				{#each ALL_SOURCES as source (source)}
					<button
						class="flex w-full items-center gap-2 px-2 py-1 text-left text-[11px] transition-colors hover:bg-surface-base"
						role="option"
						aria-selected={filters.sources.has(source)}
						onclick={() => toggleSource(source)}
					>
						<span
							class="size-3 shrink-0 rounded-sm border border-border {filters.sources.has(source) ? 'bg-blue-500 border-blue-500' : 'bg-transparent'}"
						></span>
						<span class="text-content-base">{source}</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Level checkbox group (inline, no dropdown needed — only 5 values) -->
	<div class="flex items-center gap-1.5" role="group" aria-label="Level filter">
		{#each ALL_LEVELS as level (level)}
			<label class="flex cursor-pointer items-center gap-1">
				<input
					type="checkbox"
					class="sr-only"
					checked={filters.levels.has(level)}
					onchange={() => toggleLevel(level)}
				/>
				<span
					class="flex h-4 w-4 shrink-0 items-center justify-center rounded border {filters.levels.has(level) ? 'border-blue-500 bg-blue-500' : 'border-border bg-transparent'}"
					aria-hidden="true"
				>
					{#if filters.levels.has(level)}
						<span class="text-[8px] text-white font-bold leading-none">✓</span>
					{/if}
				</span>
				<span class="text-[11px] {LEVEL_COLOR[level] ?? 'text-content-base'}">{level}</span>
			</label>
		{/each}
	</div>

	<!-- Category multi-select dropdown (populated from knownCategories) -->
	<div class="relative" data-category-dropdown>
		<button
			class="flex items-center gap-1 rounded border border-border bg-surface-raised px-2 py-0.5 text-[11px] text-content-base transition-colors hover:border-border/80 hover:bg-surface-raised/80"
			onclick={() => { categoryOpen = !categoryOpen; sourceOpen = false; }}
			aria-haspopup="listbox"
			aria-expanded={categoryOpen}
			disabled={knownCategories.size === 0}
		>
			Category
			{#if filters.categories.size > 0}
				<span class="rounded bg-blue-500/20 px-1 text-[10px] text-blue-400">{filters.categories.size}</span>
			{/if}
			<span class="text-content-muted text-[9px]">{categoryOpen ? "▲" : "▼"}</span>
		</button>

		{#if categoryOpen && knownCategories.size > 0}
			<div
				class="absolute left-0 top-full z-10 mt-1 max-h-48 min-w-[160px] overflow-y-auto rounded border border-border bg-surface-raised shadow-lg"
				role="listbox"
				aria-multiselectable="true"
				aria-label="Category filter"
			>
				{#each [...knownCategories].sort() as category (category)}
					<button
						class="flex w-full items-center gap-2 px-2 py-1 text-left text-[11px] transition-colors hover:bg-surface-base"
						role="option"
						aria-selected={filters.categories.has(category)}
						onclick={() => toggleCategory(category)}
					>
						<span
							class="size-3 shrink-0 rounded-sm border border-border {filters.categories.has(category) ? 'bg-blue-500 border-blue-500' : 'bg-transparent'}"
						></span>
						<span class="truncate text-content-base">{category}</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Full-text search input. The id allows the global Ctrl+F handler in
	     +layout.svelte to focus this element from outside this component. -->
	<input
		id="log-search-input"
		type="search"
		class="h-6 min-w-[180px] flex-1 rounded border border-border bg-surface-raised px-2 text-[11px] text-content-base placeholder-content-muted outline-none transition-colors focus:border-blue-500/60 focus:ring-0"
		placeholder="Search messages…"
		bind:value={filters.searchText}
		aria-label="Search log messages"
	/>

	<!-- Clear all filters button — only visible when a filter is active -->
	{#if hasActiveFilters}
		<button
			class="rounded border border-border px-2 py-0.5 text-[11px] text-content-muted transition-colors hover:border-red-500/40 hover:bg-red-500/10 hover:text-red-400"
			onclick={clearFilters}
			aria-label="Clear all filters"
		>
			Clear filters
		</button>
	{/if}
</div>
