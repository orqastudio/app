<script lang="ts">
	import {
		Icon,
		Badge,
		LoadingSpinner,
		ErrorDisplay,
		EmptyState,
		Caption,
		Button,
		Center,
		HStack,
		Stack,
		Text,
		Box,
		ScrollArea,
	} from "@orqastudio/svelte-components/pure";
	import type { Lesson } from "@orqastudio/types";
	import { getStores } from "@orqastudio/sdk";

	const { pluginRegistry } = getStores();

	/**
	 * Returns Tailwind class string for a lesson category badge.
	 * Derives color from the lesson schema's categories declared in the plugin manifest.
	 * Falls back to muted when the category has no declared color.
	 * @param category - The lesson category key.
	 * @returns A Tailwind class string for badge styling.
	 */
	function categoryColor(category: string): string {
		const cats = pluginRegistry.getSchemaCategories("lesson");
		const cat = cats.find((c) => c.key === category);
		if (cat?.color) return `bg-[${cat.color}]/10 text-[${cat.color}]`;
		return "bg-muted text-muted-foreground";
	}

	let {
		lessons,
		loading,
		error,
		selectedId,
		onSelect,
		onRetry,
	}: {
		lessons: Lesson[];
		loading: boolean;
		error: string | null;
		selectedId: string | null;
		onSelect: (lesson: Lesson) => void;
		onRetry: () => void;
	} = $props();

	const activeCount = $derived(lessons.filter((l) => l.status === "active").length);
	const promotedCount = $derived(lessons.filter((l) => l.status === "promoted").length);
	const promotionCandidates = $derived(
		lessons.filter((l) => l.recurrence >= 2 && l.status === "active"),
	);

	function statusVariant(status: string): "default" | "secondary" | "outline" {
		switch (status) {
			case "promoted":
				return "default";
			case "resolved":
				return "secondary";
			default:
				return "outline";
		}
	}
</script>

<Stack gap={0} height="full">
	<!-- Header -->
	<HStack justify="between" borderBottom paddingX={3} paddingY={2}>
		<HStack gap={2}>
			<Icon name="book-open" size="md" />
			<Text variant="body-strong">Lessons</Text>
		</HStack>
		<HStack gap={1}>
			{#if promotionCandidates.length > 0}
				<Badge variant="secondary" size="sm">
					<Icon name="trending-up" size="xs" />
					{promotionCandidates.length} ready to promote
				</Badge>
			{/if}
			{#if promotedCount > 0}
				<Badge variant="outline" size="sm">
					<Icon name="check-circle" size="xs" />
					{promotedCount} promoted
				</Badge>
			{/if}
		</HStack>
	</HStack>

	<ScrollArea full>
		<Box padding={2}>
			{#if loading && lessons.length === 0}
				<Center padding={8}>
					<LoadingSpinner />
				</Center>
			{:else if error}
				<ErrorDisplay message="Failed to load lessons: {error}" {onRetry} />
			{:else if lessons.length === 0}
				<EmptyState
					icon="book-open"
					title="No lessons yet"
					description="Lessons are captured when patterns recur across agent sessions."
				/>
			{:else}
				<!-- Active lessons -->
				{#if activeCount > 0}
					<Text variant="overline-muted" block>
						Active ({activeCount})
					</Text>
					<Stack gap={1} marginTop={1}>
						{#each lessons.filter((l) => l.status === "active") as lesson (lesson.id)}
							<Button
								variant="ghost"
								size="sm"
								full
								onclick={() => onSelect(lesson)}
								aria-pressed={selectedId === lesson.id}
								style="justify-content: flex-start; text-align: left; height: auto; padding: 0.5rem;"
							>
								<HStack justify="between" gap={1} align="start" full>
									<div style="min-width: 0; flex: 1; display: flex; flex-direction: column;">
										<HStack gap={1}>
											<Caption variant="caption-mono">{lesson.id}</Caption>
											<span class={`rounded px-1 py-0.5 text-[10px] font-medium ${categoryColor(lesson.category)}`}>
												{lesson.category}
											</span>
										</HStack>
										<Caption truncate>{lesson.title}</Caption>
									</div>
									{#if lesson.recurrence >= 2}
										<Badge variant="secondary" size="xs">
											x{lesson.recurrence}
										</Badge>
									{/if}
								</HStack>
							</Button>
						{/each}
					</Stack>
				{/if}

				<!-- Promoted lessons -->
				{#if promotedCount > 0}
					<Text variant="overline-muted" block>
						Promoted ({promotedCount})
					</Text>
					<Stack gap={1} marginTop={1}>
						{#each lessons.filter((l) => l.status === "promoted") as lesson (lesson.id)}
							<Button
								variant="ghost"
								size="sm"
								full
								onclick={() => onSelect(lesson)}
								aria-pressed={selectedId === lesson.id}
								style="justify-content: flex-start; text-align: left; height: auto; padding: 0.5rem;"
							>
								<div style="min-width: 0; flex: 1; display: flex; flex-direction: column;">
									<HStack gap={1}>
										<Caption variant="caption-mono">{lesson.id}</Caption>
										<Badge variant={statusVariant(lesson.status)} size="xs">
											{lesson.status}
										</Badge>
									</HStack>
									<Caption truncate>{lesson.title}</Caption>
								</div>
							</Button>
						{/each}
					</Stack>
				{/if}

				<!-- Resolved lessons -->
				{#if lessons.some((l) => l.status === "resolved")}
					<Text variant="overline-muted" block>
						Resolved
					</Text>
					<Stack gap={1}>
						{#each lessons.filter((l) => l.status === "resolved") as lesson (lesson.id)}
							<Button
								variant="ghost"
								size="sm"
								full
								onclick={() => onSelect(lesson)}
								aria-pressed={selectedId === lesson.id}
								style="justify-content: flex-start; text-align: left; height: auto; padding: 0.5rem; opacity: {selectedId === lesson.id ? 1 : 0.6};"
							>
								<HStack gap={1}>
									<Caption variant="caption-mono">{lesson.id}</Caption>
									<Caption truncate>{lesson.title}</Caption>
								</HStack>
							</Button>
						{/each}
					</Stack>
				{/if}
			{/if}
		</Box>
	</ScrollArea>
</Stack>
