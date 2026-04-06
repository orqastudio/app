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
		ScrollArea,
		Panel,
		SectionHeader,
		Callout,
		Separator,
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
	<Panel padding="normal" border="bottom">
		<Stack gap={2}>
			<HStack justify="between" gap={3} align="start">
				<Stack gap={1} flex={1} minHeight={0}>
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
				</Stack>

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
				<Callout tone="warning" density="compact" iconName="arrow-up-circle">
					<Caption tone="warning">Recurred {lesson.recurrence} times — ready for promotion to a rule</Caption>
				</Callout>
			{/if}

			{#if lesson.promoted_to}
				<Callout tone="muted" density="compact" iconName="external-link">
					<Caption>Promoted to: <Code>{lesson.promoted_to}</Code></Caption>
				</Callout>
			{/if}
		</Stack>
	</Panel>

	<!-- Metadata row -->
	<SectionHeader variant="compact">
		<HStack gap={3}>
			<Caption>Created: {lesson.created}</Caption>
			<Separator orientation="vertical" />
			<Caption>Updated: {lesson.updated}</Caption>
			<Separator orientation="vertical" />
			<Caption variant="caption-mono">{lesson.file_path}</Caption>
		</HStack>
	</SectionHeader>

	<!-- Body -->
	<ScrollArea full>
		<Panel padding="normal">
			<MarkdownRenderer content={lesson.body} codeRenderer={DiagramCodeBlock} linkRenderer={MarkdownLink} />
		</Panel>
	</ScrollArea>
</Stack>
