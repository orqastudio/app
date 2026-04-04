<script lang="ts">
	import {
		Icon,
		Badge,
		Heading,
		Caption,
		Code,
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

<div class="flex h-full flex-col">
	<!-- Header -->
	<div class="border-b border-border px-4 py-3">
		<div class="flex items-start justify-between gap-3">
			<div class="min-w-0 flex-1">
				<div class="mb-1 flex items-center gap-2">
					<span class="font-mono text-xs text-muted-foreground">{lesson.id}</span>
					<span
						class="rounded px-1.5 py-0.5 text-[11px] font-medium {categoryColor(lesson.category)}"
					>
						{lesson.category}
					</span>
					{#if lesson.status !== "active"}
						<Badge variant="secondary" size="xs">
							{lesson.status}
						</Badge>
					{/if}
				</div>
				<Heading level={5}>{lesson.title}</Heading>
			</div>

			<!-- Recurrence indicator and action -->
			<div class="flex shrink-0 flex-col items-end gap-2">
				<div class="flex items-center gap-1">
					<Icon name="trending-up" size="sm" />
					<span class="text-xs font-medium text-muted-foreground">{lesson.recurrence}x</span>
				</div>
				{#if lesson.status === "active"}
					<button
						class="h-6 rounded border border-border px-2 text-[11px] hover:bg-accent"
						onclick={() => onIncrementRecurrence(lesson.id)}
					>
						+1 Recurrence
					</button>
				{/if}
			</div>
		</div>

		{#if isPromotionCandidate}
			<div class="mt-2 flex items-center gap-1 rounded-md bg-warning/10 px-2 py-1.5">
				<Icon name="arrow-up-circle" size="sm" />
				<span class="text-xs text-warning">Recurred {lesson.recurrence} times — ready for promotion to a rule</span>
			</div>
		{/if}

		{#if lesson.promoted_to}
			<div class="mt-2 flex items-center gap-1 rounded-md bg-muted px-2 py-1.5">
				<Icon name="external-link" size="sm" />
				<Caption>Promoted to: <Code>{lesson.promoted_to}</Code></Caption>
			</div>
		{/if}
	</div>

	<!-- Metadata row -->
	<div class="flex items-center gap-3 border-b border-border px-4 py-1.5">
		<Caption>Created: {lesson.created}</Caption>
		<div class="h-3 w-px bg-border"></div>
		<Caption>Updated: {lesson.updated}</Caption>
		<div class="h-3 w-px bg-border"></div>
		<span class="font-mono text-xs text-muted-foreground">{lesson.file_path}</span>
	</div>

	<!-- Body -->
	<div class="flex-1 overflow-y-auto">
		<div class="px-4 py-4">
			<MarkdownRenderer content={lesson.body} codeRenderer={DiagramCodeBlock} linkRenderer={MarkdownLink} />
		</div>
	</div>
</div>
