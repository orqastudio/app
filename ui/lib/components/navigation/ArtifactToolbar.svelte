<script lang="ts">
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import * as Popover from "$lib/components/ui/popover";
	import { Button } from "$lib/components/ui/button";
	import ArrowUpDownIcon from "@lucide/svelte/icons/arrow-up-down";
	import FilterIcon from "@lucide/svelte/icons/filter";
	import CheckIcon from "@lucide/svelte/icons/check";
	import { statusColor } from "$lib/components/shared/StatusIndicator.svelte";
	import { countFieldValues } from "$lib/utils/artifact-view";
	import type {
		FilterableField,
		SortableField,
		SortConfig,
		NavigationConfig,
		DocNode,
	} from "$lib/types/nav-tree";

	let {
		sortableFields,
		filterableFields,
		navigationConfig,
		nodes,
		currentSort,
		currentFilters,
		currentGroup,
		onSortChange,
		onFilterChange,
		onGroupChange,
	}: {
		sortableFields: SortableField[];
		filterableFields: FilterableField[];
		navigationConfig?: NavigationConfig;
		nodes: DocNode[];
		currentSort: SortConfig;
		currentFilters: Record<string, string[]>;
		currentGroup: string | null;
		onSortChange: (sort: SortConfig) => void;
		onFilterChange: (filters: Record<string, string[]>) => void;
		onGroupChange: (group: string | null) => void;
	} = $props();

	// Derive whether filters are active (any non-empty filter array)
	const hasActiveFilters = $derived(
		Object.values(currentFilters).some((v) => v.length > 0),
	);

	// Derive default sort from navigation config
	const defaultSort = $derived(
		navigationConfig?.defaults?.sort ?? { field: "title", direction: "asc" },
	);
	const isNonDefaultSort = $derived(
		currentSort.field !== defaultSort.field ||
			currentSort.direction !== defaultSort.direction,
	);

	// The radio group value encodes "field:direction"
	const sortValue = $derived(`${currentSort.field}:${currentSort.direction}`);

	function setSortFromValue(value: string) {
		const [field, direction] = value.split(":");
		if (field && direction) {
			onSortChange({ field, direction });
		}
	}

	function humanizeField(name: string): string {
		return name
			.replace(/[-_]/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	function humanizeValue(value: string): string {
		return value
			.replace(/[-_]/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	function isFilterActive(field: string, value: string): boolean {
		return (currentFilters[field] ?? []).includes(value);
	}

	function toggleFilter(field: string, value: string) {
		const current = currentFilters[field] ?? [];
		const updated = current.includes(value)
			? current.filter((v) => v !== value)
			: [...current, value];
		onFilterChange({ ...currentFilters, [field]: updated });
	}

	function clearFieldFilters(field: string) {
		onFilterChange({ ...currentFilters, [field]: [] });
	}

	function clearAllFilters() {
		const cleared: Record<string, string[]> = {};
		for (const key of Object.keys(currentFilters)) {
			cleared[key] = [];
		}
		onFilterChange(cleared);
	}

	// Sort options
	interface SortOption {
		label: string;
		value: string;
	}

	const sortOptions = $derived<SortOption[]>([
		{ label: "Title (A-Z)", value: "title:asc" },
		{ label: "Title (Z-A)", value: "title:desc" },
		...sortableFields.filter((f) => f.name !== "title").flatMap((f) => {
			const human = humanizeField(f.name);
			if (f.field_type === "date" || f.field_type === "datetime") {
				return [
					{ label: `${human} (newest)`, value: `${f.name}:desc` },
					{ label: `${human} (oldest)`, value: `${f.name}:asc` },
				];
			}
			return [
				{ label: `${human} (A-Z)`, value: `${f.name}:asc` },
				{ label: `${human} (Z-A)`, value: `${f.name}:desc` },
			];
		}),
	]);
</script>

<div class="flex h-10 items-center justify-end gap-1 border-b border-border px-2">
	<!-- Sort dropdown -->
	<div class="relative">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				{#snippet child({ props })}
					<Button
						{...props}
						variant="ghost"
						size="icon-sm"
						class="text-muted-foreground hover:text-foreground {isNonDefaultSort ? 'text-foreground' : ''}"
					>
						<ArrowUpDownIcon class="h-3.5 w-3.5" />
					</Button>
				{/snippet}
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="start" class="w-52">
				<DropdownMenu.Label class="text-xs text-muted-foreground">Sort by</DropdownMenu.Label>
				<DropdownMenu.RadioGroup value={sortValue} onValueChange={setSortFromValue}>
					{#each sortOptions as option (option.value)}
						<DropdownMenu.RadioItem value={option.value}>
							{option.label}
						</DropdownMenu.RadioItem>
					{/each}
				</DropdownMenu.RadioGroup>

				{#if filterableFields.length > 0}
					<DropdownMenu.Separator />
					<DropdownMenu.Label class="text-xs text-muted-foreground">Group by</DropdownMenu.Label>
					<DropdownMenu.Item onclick={() => onGroupChange(null)}>
						<span class="flex items-center gap-2">
							{#if currentGroup === null}
								<CheckIcon class="h-3.5 w-3.5" />
							{:else}
								<span class="h-3.5 w-3.5"></span>
							{/if}
							None
						</span>
					</DropdownMenu.Item>
					{#each filterableFields as field (field.name)}
						<DropdownMenu.Item onclick={() => onGroupChange(field.name)}>
							<span class="flex items-center gap-2">
								{#if currentGroup === field.name}
									<CheckIcon class="h-3.5 w-3.5" />
								{:else}
									<span class="h-3.5 w-3.5"></span>
								{/if}
								{humanizeField(field.name)}
							</span>
						</DropdownMenu.Item>
					{/each}
				{/if}
			</DropdownMenu.Content>
		</DropdownMenu.Root>
		{#if isNonDefaultSort}
			<span
				class="pointer-events-none absolute right-0.5 top-0.5 h-1.5 w-1.5 rounded-full bg-blue-500"
			></span>
		{/if}
	</div>

	<!-- Filter popover -->
	{#if filterableFields.length > 0}
		<div class="relative">
			<Popover.Root>
				<Popover.Trigger>
					{#snippet child({ props })}
						<Button
							{...props}
							variant="ghost"
							size="icon-sm"
							class="text-muted-foreground hover:text-foreground {hasActiveFilters ? 'text-foreground' : ''}"
						>
							<FilterIcon class="h-3.5 w-3.5" />
						</Button>
					{/snippet}
				</Popover.Trigger>
				<Popover.Content align="start" class="w-64 p-0">
					<div class="flex flex-col">
						{#each filterableFields as field (field.name)}
							{@const counts = countFieldValues(nodes, field.name)}
							<div class="border-b border-border last:border-0">
								<div class="flex items-center justify-between px-3 py-2">
									<span class="text-[10px] font-semibold uppercase tracking-wide text-muted-foreground">
										{humanizeField(field.name)}
									</span>
									{#if (currentFilters[field.name] ?? []).length > 0}
										<button
											class="text-[10px] text-muted-foreground hover:text-foreground"
											onclick={() => clearFieldFilters(field.name)}
										>
											Clear
										</button>
									{/if}
								</div>
								<div class="flex flex-col pb-1">
									{#each field.values as value (value)}
										{@const active = isFilterActive(field.name, value)}
										{@const count = counts[value] ?? 0}
										<button
											class="flex items-center gap-2 px-3 py-1.5 text-left text-sm hover:bg-accent/50"
											onclick={() => toggleFilter(field.name, value)}
										>
											<!-- Checkbox indicator -->
											<span
												class="flex h-3.5 w-3.5 shrink-0 items-center justify-center rounded-sm border {active
													? 'border-primary bg-primary'
													: 'border-muted-foreground/40'}"
											>
												{#if active}
													<CheckIcon class="h-2.5 w-2.5 text-primary-foreground" />
												{/if}
											</span>
											<!-- Status dot if this is a status field -->
											{#if field.name === "status"}
												<span
													class="h-2 w-2 shrink-0 rounded-full {statusColor(value)}"
												></span>
											{/if}
											<span class="flex-1 capitalize">{humanizeValue(value)}</span>
											{#if count > 0}
												<span class="tabular-nums text-[10px] text-muted-foreground">{count}</span>
											{/if}
										</button>
									{/each}
								</div>
							</div>
						{/each}

						{#if hasActiveFilters}
							<div class="border-t border-border p-2">
								<button
									class="w-full rounded px-2 py-1.5 text-center text-xs text-muted-foreground hover:bg-accent/50 hover:text-foreground"
									onclick={clearAllFilters}
								>
									Clear all filters
								</button>
							</div>
						{/if}
					</div>
				</Popover.Content>
			</Popover.Root>
			{#if hasActiveFilters}
				<span
					class="pointer-events-none absolute right-0.5 top-0.5 h-1.5 w-1.5 rounded-full bg-blue-500"
				></span>
			{/if}
		</div>
	{/if}
</div>
