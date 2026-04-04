<script lang="ts">
	import {
		Icon,
		Badge,
		Heading,
		Caption,
		Code,
		Button,
		HStack,
		Stack,
		Text,
		Box,
		ScrollArea,
	} from "@orqastudio/svelte-components/pure";
	import { MarkdownRenderer } from "@orqastudio/svelte-components/connected";
	import DiagramCodeBlock from "$lib/components/content/DiagramCodeBlock.svelte";
	import MarkdownLink from "$lib/components/content/MarkdownLink.svelte";
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
		lesson,
		onIncrementRecurrence,
	}: {
		lesson: Lesson;
		onIncrementRecurrence: (id: string) => void;
	} = $props();

	const isPromotionCandidate = $derived(lesson.recurrence >= 2 && lesson.status === "active");
</script>

<Stack gap={0} height="full">
	<!-- Header -->
	<Box borderBottom paddingX={4} paddingY={3}>
		<HStack justify="between" gap={3} align="start">
			<div style="min-width: 0; flex: 1; display: flex; flex-direction: column; gap: 0.25rem;">
				<HStack gap={2}>
					<Caption variant="caption-mono">{lesson.id}</Caption>
					<span class={`rounded px-1.5 py-0.5 text-[11px] font-medium ${categoryColor(lesson.category)}`}>
						{lesson.category}
					</span>
					{#if lesson.status !== "active"}
						<Badge variant="secondary" size="xs">
							{lesson.status}
						</Badge>
					{/if}
				</HStack>
				<Heading level={5}>{lesson.title}</Heading>
			</div>

			<!-- Recurrence indicator and action -->
			<Stack gap={2} align="end" flex={0}>
				<HStack gap={1}>
					<Icon name="trending-up" size="sm" />
					<Caption variant="caption-strong">{lesson.recurrence}x</Caption>
				</HStack>
				{#if lesson.status === "active"}
					<Button
						variant="outline"
						size="sm"
						onclick={() => onIncrementRecurrence(lesson.id)}
					>
						+1 Recurrence
					</Button>
				{/if}
			</Stack>
		</HStack>

		{#if isPromotionCandidate}
			<HStack gap={1} marginTop={2} style="border-radius: 0.375rem; background: hsl(var(--warning) / 0.1); padding: 0.375rem 0.5rem;">
				<Icon name="arrow-up-circle" size="sm" />
				<Caption tone="warning">Recurred {lesson.recurrence} times — ready for promotion to a rule</Caption>
			</HStack>
		{/if}

		{#if lesson.promoted_to}
			<HStack gap={1} marginTop={2} style="border-radius: 0.375rem; background: hsl(var(--muted)); padding: 0.375rem 0.5rem;">
				<Icon name="external-link" size="sm" />
				<Caption>Promoted to: <Code>{lesson.promoted_to}</Code></Caption>
			</HStack>
		{/if}
	</Box>

	<!-- Metadata row -->
	<HStack gap={3} borderBottom paddingX={4} paddingY={1}>
		<Caption>Created: {lesson.created}</Caption>
		<div style="height: 0.75rem; width: 1px; background: hsl(var(--border));"></div>
		<Caption>Updated: {lesson.updated}</Caption>
		<div style="height: 0.75rem; width: 1px; background: hsl(var(--border));"></div>
		<Caption variant="caption-mono">{lesson.file_path}</Caption>
	</HStack>

	<!-- Body -->
	<ScrollArea full>
		<Box padding={4}>
			<MarkdownRenderer content={lesson.body} codeRenderer={DiagramCodeBlock} linkRenderer={MarkdownLink} />
		</Box>
	</ScrollArea>
</Stack>
