<script lang="ts">
	import {
		Icon,
		Badge,
		LoadingSpinner,
		ErrorDisplay,
		EmptyState,
		Caption,
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

<div class="flex h-full flex-col">
	<!-- Header -->
	<div class="flex items-center justify-between border-b border-border px-3 py-2">
		<div class="flex items-center gap-2">
			<Icon name="book-open" size="md" />
			<span class="text-sm font-medium">Lessons</span>
		</div>
		<div class="flex items-center gap-1">
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
		</div>
	</div>

	<div class="flex-1 overflow-y-auto">
		<div class="p-2">
			{#if loading && lessons.length === 0}
				<div class="flex justify-center py-8">
					<LoadingSpinner />
				</div>
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
					<span class="mb-1.5 block px-1 text-xs font-medium uppercase tracking-wide text-muted-foreground">
						Active ({activeCount})
					</span>
					<div class="mb-3 space-y-1">
						{#each lessons.filter((l) => l.status === "active") as lesson (lesson.id)}
							<button
								class="w-full rounded-md px-2 py-2 text-left {selectedId === lesson.id ? 'bg-accent' : 'hover:bg-accent/50'}"
								onclick={() => onSelect(lesson)}
							>
								<div class="flex items-start justify-between gap-2">
									<div class="min-w-0 flex-1">
										<div class="flex items-center gap-1">
											<span class="font-mono text-xs text-muted-foreground">{lesson.id}</span>
											<span
												class="rounded px-1 py-0.5 text-[10px] font-medium {categoryColor(lesson.category)}"
											>
												{lesson.category}
											</span>
										</div>
										<p class="mt-0.5 truncate text-xs font-medium">{lesson.title}</p>
									</div>
									<div class="flex shrink-0 flex-col items-end gap-1">
										{#if lesson.recurrence >= 2}
											<Badge variant="secondary" size="xs">
												x{lesson.recurrence}
											</Badge>
										{/if}
									</div>
								</div>
							</button>
						{/each}
					</div>
				{/if}

				<!-- Promoted lessons -->
				{#if promotedCount > 0}
					<span class="mb-1.5 block px-1 text-xs font-medium uppercase tracking-wide text-muted-foreground">
						Promoted ({promotedCount})
					</span>
					<div class="mb-3 space-y-1">
						{#each lessons.filter((l) => l.status === "promoted") as lesson (lesson.id)}
							<button
								class="w-full rounded-md px-2 py-2 text-left {selectedId === lesson.id ? 'bg-accent' : 'hover:bg-accent/50'}"
								onclick={() => onSelect(lesson)}
							>
								<div class="flex items-start justify-between gap-2">
									<div class="min-w-0 flex-1">
										<div class="flex items-center gap-1">
											<span class="font-mono text-xs text-muted-foreground">{lesson.id}</span>
											<Badge variant={statusVariant(lesson.status)} size="xs">
												{lesson.status}
											</Badge>
										</div>
										<Caption truncate>
											{lesson.title}
										</Caption>
									</div>
								</div>
							</button>
						{/each}
					</div>
				{/if}

				<!-- Resolved lessons -->
				{#if lessons.some((l) => l.status === "resolved")}
					<span class="mb-1.5 block px-1 text-xs font-medium uppercase tracking-wide text-muted-foreground">
						Resolved
					</span>
					<div class="space-y-1">
						{#each lessons.filter((l) => l.status === "resolved") as lesson (lesson.id)}
							<button
								class="w-full rounded-md px-2 py-2 text-left {selectedId === lesson.id ? 'bg-accent' : 'opacity-60 hover:bg-accent/50 hover:opacity-100'}"
								onclick={() => onSelect(lesson)}
							>
								<div class="flex items-center gap-1">
									<span class="font-mono text-xs text-muted-foreground">{lesson.id}</span>
									<Caption truncate>{lesson.title}</Caption>
								</div>
							</button>
						{/each}
					</div>
				{/if}
			{/if}
		</div>
	</div>
</div>
