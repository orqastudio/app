<script lang="ts">
	import {
		Icon,
		DropdownMenuRoot,
		DropdownMenuTrigger,
		DropdownMenuItem,
		DropdownMenuContent,
		DropdownMenuSeparator,
		DropdownMenuLabel,
		DropdownMenuRadioGroup,
		DropdownMenuRadioItem,
		PopoverRoot as Popover,
		PopoverTrigger,
		PopoverContent,
		Button,
		HStack,
		Stack,
		Box,
		Caption,
		Text,
		Panel,
		SectionHeader,
		SectionFooter,
		Dot,
		CheckIndicator,
		statusIconName,
	} from "@orqastudio/svelte-components/pure";
	import { countFieldValues } from "$lib/utils/artifact-view";
	import type {
		FilterableField,
		SortableField,
		SortConfig,
		NavigationConfig,
		DocNode,
	} from "@orqastudio/types";

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
		sortableFields: readonly SortableField[];
		filterableFields: readonly FilterableField[];
		navigationConfig?: NavigationConfig;
		nodes: readonly DocNode[];
		currentSort: Readonly<SortConfig>;
		currentFilters: Readonly<Record<string, readonly string[]>>;
		currentGroup: string | null;
		onSortChange: (sort: SortConfig) => void;
		onFilterChange: (filters: Record<string, readonly string[]>) => void;
		onGroupChange: (group: string | null) => void;
	} = $props();

	// Derive whether filters are active (any non-empty filter array)
	const hasActiveFilters = $derived(Object.values(currentFilters).some((v) => v.length > 0));

	// Derive default sort from navigation config
	const defaultSort = $derived(
		navigationConfig?.defaults?.sort ?? { field: "title", direction: "asc" },
	);
	const isNonDefaultSort = $derived(
		currentSort.field !== defaultSort.field || currentSort.direction !== defaultSort.direction,
	);

	// The radio group value encodes "field:direction"
	const sortValue = $derived(`${currentSort.field}:${currentSort.direction}`);

	/**
	 * Parse a "field:direction" radio value and propagate the new sort config upward.
	 * @param value - The encoded sort string in "field:direction" format (e.g. "title:asc").
	 */
	function setSortFromValue(value: string) {
		const [field, direction] = value.split(":");
		if (field && direction) {
			onSortChange({ field, direction });
		}
	}

	/**
	 * Convert a field key into a human-readable label by replacing separators and title-casing.
	 * @param name - The raw field key such as "created_at" or "artifact-type".
	 * @returns A formatted label such as "Created At" or "Artifact Type".
	 */
	function humanizeField(name: string): string {
		return name.replace(/[-_]/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
	}

	/**
	 * Convert a filter value into a human-readable label by replacing separators and title-casing.
	 * @param value - The raw filter value such as "in_progress" or "high-priority".
	 * @returns A formatted label such as "In Progress" or "High Priority".
	 */
	function humanizeValue(value: string): string {
		return value.replace(/[-_]/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
	}

	/**
	 * Return true when the given field value is currently selected as a filter.
	 * @param field - The filterable field key to check.
	 * @param value - The specific field value to test for active selection.
	 * @returns True if the value is in the current filter set for the field.
	 */
	function isFilterActive(field: string, value: string): boolean {
		return (currentFilters[field] ?? []).includes(value);
	}

	/**
	 * Toggle a filter value on or off and propagate the updated filter map upward.
	 * @param field - The filterable field key whose filter is being toggled.
	 * @param value - The specific field value to add or remove from the active filters.
	 */
	function toggleFilter(field: string, value: string) {
		const current = currentFilters[field] ?? [];
		const updated: readonly string[] = current.includes(value)
			? current.filter((v) => v !== value)
			: [...current, value];
		onFilterChange({ ...currentFilters, [field]: updated } as Record<string, readonly string[]>);
	}

	/**
	 * Clear all active filters for a specific field and propagate the updated filter map upward.
	 * @param field - The filterable field key whose filters should be cleared.
	 */
	function clearFieldFilters(field: string) {
		onFilterChange({ ...currentFilters, [field]: [] as readonly string[] } as Record<
			string,
			readonly string[]
		>);
	}

	/** Clear all active filters across every field and propagate the empty filter map upward. */
	function clearAllFilters() {
		const cleared: Record<string, readonly string[]> = {};
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
		...sortableFields
			.filter((f) => f.name !== "title")
			.flatMap((f) => {
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

<SectionHeader variant="compact">
	{#snippet end()}
		<!-- Sort dropdown -->
		<Box position="relative">
			<DropdownMenuRoot>
				<DropdownMenuTrigger>
					{#snippet child({ props })}
						<Button {...props} variant="ghost" size="icon-sm">
							<Icon name="arrow-up-down" size="sm" />
						</Button>
					{/snippet}
				</DropdownMenuTrigger>
				<DropdownMenuContent align="start">
					<DropdownMenuLabel>Sort by</DropdownMenuLabel>
					<DropdownMenuRadioGroup value={sortValue} onValueChange={setSortFromValue}>
						{#each sortOptions as option (option.value)}
							<DropdownMenuRadioItem value={option.value}>
								{option.label}
							</DropdownMenuRadioItem>
						{/each}
					</DropdownMenuRadioGroup>

					{#if filterableFields.length > 0}
						<DropdownMenuSeparator />
						<DropdownMenuLabel>Group by</DropdownMenuLabel>
						<DropdownMenuItem onclick={() => onGroupChange(null)}>
							<HStack gap={2}>
								{#if currentGroup === null}
									<Icon name="check" size="sm" />
								{:else}
									<Box size="icon-sm" />
								{/if}
								None
							</HStack>
						</DropdownMenuItem>
						{#each filterableFields as field (field.name)}
							<DropdownMenuItem onclick={() => onGroupChange(field.name)}>
								<HStack gap={2}>
									{#if currentGroup === field.name}
										<Icon name="check" size="sm" />
									{:else}
										<Box size="icon-sm" />
									{/if}
									{humanizeField(field.name)}
								</HStack>
							</DropdownMenuItem>
						{/each}
					{/if}
				</DropdownMenuContent>
			</DropdownMenuRoot>
			{#if isNonDefaultSort}
				<Box position="absolute" top={0.5} right={0.5}>
					<Dot size="xs" color="primary" />
				</Box>
			{/if}
		</Box>

		<!-- Filter popover -->
		{#if filterableFields.length > 0}
			<Box position="relative">
				<Popover>
					<PopoverTrigger>
						{#snippet child({ props })}
							<Button {...props} variant="ghost" size="icon-sm">
								<Icon name="filter" size="sm" />
							</Button>
						{/snippet}
					</PopoverTrigger>
					<PopoverContent align="start">
						<Stack gap={0}>
							{#each filterableFields as field (field.name)}
								{@const counts = countFieldValues(nodes, field.name)}
								<Panel padding="tight" border="bottom">
									<HStack justify="between">
										<Caption>
											{humanizeField(field.name)}
										</Caption>
										{#if (currentFilters[field.name] ?? []).length > 0}
											<Button
												variant="ghost"
												size="sm"
												onclick={() => clearFieldFilters(field.name)}
											>
												Clear
											</Button>
										{/if}
									</HStack>
									<Stack gap={0}>
										{#each field.values as value (value)}
											{@const active = isFilterActive(field.name, value)}
											{@const count = counts[value] ?? 0}
											<Button variant="ghost" onclick={() => toggleFilter(field.name, value)}>
												<CheckIndicator checked={active} />
												{#if field.name === "status"}
													<Icon name={statusIconName(value)} size="sm" />
												{/if}
												<Box flex={1}><Text variant="body">{humanizeValue(value)}</Text></Box>
												{#if count > 0}
													<Caption>{count}</Caption>
												{/if}
											</Button>
										{/each}
									</Stack>
								</Panel>
							{/each}

							{#if hasActiveFilters}
								<SectionFooter variant="compact">
									<Button variant="ghost" onclick={clearAllFilters}>Clear all filters</Button>
								</SectionFooter>
							{/if}
						</Stack>
					</PopoverContent>
				</Popover>
				{#if hasActiveFilters}
					<Box position="absolute" top={0.5} right={0.5}>
						<Dot size="xs" color="primary" />
					</Box>
				{/if}
			</Box>
		{/if}
	{/snippet}
</SectionHeader>
