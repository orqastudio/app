<script lang="ts">
	import { Icon, Badge } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { ScrollArea } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
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
						<Badge variant="secondary" class="text-[11px] px-1.5 py-0">
							{lesson.status}
						</Badge>
					{/if}
				</div>
				<h2 class="text-sm font-semibold leading-snug">{lesson.title}</h2>
			</div>

			<!-- Recurrence indicator and action -->
			<div class="flex shrink-0 flex-col items-end gap-2">
				<div class="flex items-center gap-1.5">
					<Icon name="trending-up" size="sm" />
					<span class="text-xs font-medium">{lesson.recurrence}x</span>
				</div>
				{#if lesson.status === "active"}
					<Button
						variant="outline"
						size="sm"
						class="h-6 text-[11px]"
						onclick={() => onIncrementRecurrence(lesson.id)}
					>
						+1 Recurrence
					</Button>
				{/if}
			</div>
		</div>

		{#if isPromotionCandidate}
			<div
				class="mt-2 flex items-center gap-1.5 rounded-md bg-warning/10 px-2 py-1.5 text-xs text-warning"
			>
				<Icon name="arrow-up-circle" size="sm" />
				<span>Recurred {lesson.recurrence} times — ready for promotion to a rule</span>
			</div>
		{/if}

		{#if lesson.promoted_to}
			<div
				class="mt-2 flex items-center gap-1.5 rounded-md bg-muted px-2 py-1.5 text-xs text-muted-foreground"
			>
				<Icon name="external-link" size="sm" />
				<span>Promoted to: <code class="font-mono">{lesson.promoted_to}</code></span>
			</div>
		{/if}
	</div>

	<!-- Metadata row -->
	<div class="flex items-center gap-3 border-b border-border px-4 py-1.5">
		<span class="text-xs text-muted-foreground">Created: {lesson.created}</span>
		<Separator orientation="vertical" class="h-3" />
		<span class="text-xs text-muted-foreground">Updated: {lesson.updated}</span>
		<Separator orientation="vertical" class="h-3" />
		<span class="font-mono text-xs text-muted-foreground">{lesson.file_path}</span>
	</div>

	<!-- Body -->
	<ScrollArea class="flex-1">
		<div class="px-4 py-4">
			<MarkdownRenderer content={lesson.body} />
		</div>
	</ScrollArea>
</div>
