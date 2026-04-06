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
		CategoryBadge,
	} from "@orqastudio/svelte-components/pure";
	import { MarkdownRenderer } from "@orqastudio/svelte-components/connected";
	import DiagramCodeBlock from "$lib/components/content/DiagramCodeBlock.svelte";
	import MarkdownLink from "$lib/components/content/MarkdownLink.svelte";
	import type { Lesson } from "@orqastudio/types";
	import { getStores } from "@orqastudio/sdk";

	const { pluginRegistry } = getStores();

	/**
	 * Resolve the hex color for a lesson category from the plugin registry.
	 * Returns undefined when no color is declared, letting CategoryBadge fall back to muted.
	 * @param category - The lesson category key.
	 * @returns A hex color string or undefined.
	 */
	function categoryHexColor(category: string): string | undefined {
		const cats = pluginRegistry.getSchemaCategories("lesson");
		const cat = cats.find((c) => c.key === category);
		return cat?.color;
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
						<CategoryBadge category={lesson.category} color={categoryHexColor(lesson.category)} />
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
						<Button variant="outline" size="sm" onclick={() => onIncrementRecurrence(lesson.id)}>
							+1 Recurrence
						</Button>
					{/if}
				</Stack>
			</HStack>

			{#if isPromotionCandidate}
				<Callout tone="warning" density="compact" iconName="arrow-up-circle">
					<Caption tone="warning"
						>Recurred {lesson.recurrence} times — ready for promotion to a rule</Caption
					>
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
			<MarkdownRenderer
				content={lesson.body}
				codeRenderer={DiagramCodeBlock}
				linkRenderer={MarkdownLink}
			/>
		</Panel>
	</ScrollArea>
</Stack>
