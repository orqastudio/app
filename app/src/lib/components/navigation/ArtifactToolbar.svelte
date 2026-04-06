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
		Panel,
		SectionFooter,
		statusIconName,
		resolveIcon,
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
	 *
	 * @param value
	 */
	function setSortFromValue(value: string) {
		const [field, direction] = value.split(":");
		if (field && direction) {
			onSortChange({ field, direction });
		}
	}

	/**
	 *
	 * @param name
	 */
	function humanizeField(name: string): string {
		return name.replace(/[-_]/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
	}

	/**
	 *
	 * @param value
	 */
	function humanizeValue(value: string): string {
		return value.replace(/[-_]/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
	}

	/**
	 *
	 * @param field
	 * @param value
	 */
	function isFilterActive(field: string, value: string): boolean {
		return (currentFilters[field] ?? []).includes(value);
	}

	/**
	 *
	 * @param field
	 * @param value
	 */
	function toggleFilter(field: string, value: string) {
		const current = currentFilters[field] ?? [];
		const updated: readonly string[] = current.includes(value)
			? current.filter((v) => v !== value)
			: [...current, value];
		onFilterChange({ ...currentFilters, [field]: updated } as Record<string, readonly string[]>);
	}

	/**
	 *
	 * @param field
	 */
	function clearFieldFilters(field: string) {
		onFilterChange({ ...currentFilters, [field]: [] as readonly string[] } as Record<
			string,
			readonly string[]
		>);
	}

	/**
	 *
	 */
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

<div class="border-border flex h-10 items-center justify-end gap-1 border-b px-2">
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
								<span class="h-3.5 w-3.5"></span>
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
									<span class="h-3.5 w-3.5"></span>
								{/if}
								{humanizeField(field.name)}
							</HStack>
						</DropdownMenuItem>
					{/each}
				{/if}
			</DropdownMenuContent>
		</DropdownMenuRoot>
		{#if isNonDefaultSort}
			<span
				class="bg-primary pointer-events-none absolute top-0.5 right-0.5 h-1.5 w-1.5 rounded-full"
			></span>
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
										<Button variant="ghost" size="sm" onclick={() => clearFieldFilters(field.name)}>
											Clear
										</Button>
									{/if}
								</HStack>
								<Stack gap={0}>
									{#each field.values as value (value)}
										{@const active = isFilterActive(field.name, value)}
										{@const count = counts[value] ?? 0}
										<Button variant="ghost" onclick={() => toggleFilter(field.name, value)}>
											<!-- Checkbox indicator -->
											<span
												class="flex h-3.5 w-3.5 shrink-0 items-center justify-center rounded-sm border {active
													? 'border-primary bg-primary'
													: 'border-muted-foreground/40'}"
											>
												{#if active}
													<Icon name="check" size="md" />
												{/if}
											</span>
											<!-- Status icon if this is a status field -->
											{#if field.name === "status"}
												{@const StatusIcon = resolveIcon(statusIconName(value))}
												<StatusIcon class="text-muted-foreground h-3.5 w-3.5 shrink-0" />
											{/if}
											<span class="flex-1 capitalize">{humanizeValue(value)}</span>
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
				<span
					class="bg-primary pointer-events-none absolute top-0.5 right-0.5 h-1.5 w-1.5 rounded-full"
				></span>
			{/if}
		</Box>
	{/if}
</div>
