<script lang="ts">
	import { Icon, Badge } from "@orqastudio/svelte-components/pure";
	import { ScrollArea } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import { EmptyState } from "@orqastudio/svelte-components/pure";
	import type { Lesson } from "@orqastudio/types";
	import { categoryColor } from "$lib/config/category-colors";

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
		<div class="flex items-center gap-1.5">
			{#if promotionCandidates.length > 0}
				<Badge variant="secondary" class="text-xs px-1.5 py-0.5">
					<Icon name="trending-up" size="xs" />
					{promotionCandidates.length} ready to promote
				</Badge>
			{/if}
			{#if promotedCount > 0}
				<Badge variant="outline" class="text-xs px-1.5 py-0.5">
					<Icon name="check-circle" size="xs" />
					{promotedCount} promoted
				</Badge>
			{/if}
		</div>
	</div>

	<ScrollArea class="flex-1">
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
					<p class="mb-1.5 px-1 text-xs font-medium uppercase tracking-wide text-muted-foreground">
						Active ({activeCount})
					</p>
					<div class="mb-3 space-y-1">
						{#each lessons.filter((l) => l.status === "active") as lesson (lesson.id)}
							<button
								class="w-full rounded-md px-2 py-2 text-left transition-colors hover:bg-accent {selectedId === lesson.id ? 'bg-accent' : ''}"
								onclick={() => onSelect(lesson)}
							>
								<div class="flex items-start justify-between gap-2">
									<div class="min-w-0 flex-1">
										<div class="flex items-center gap-1.5">
											<span class="font-mono text-[11px] text-muted-foreground">{lesson.id}</span>
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
											<Badge variant="secondary" class="text-[10px] px-1 py-0">
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
					<p class="mb-1.5 px-1 text-xs font-medium uppercase tracking-wide text-muted-foreground">
						Promoted ({promotedCount})
					</p>
					<div class="mb-3 space-y-1">
						{#each lessons.filter((l) => l.status === "promoted") as lesson (lesson.id)}
							<button
								class="w-full rounded-md px-2 py-2 text-left transition-colors hover:bg-accent {selectedId === lesson.id ? 'bg-accent' : ''}"
								onclick={() => onSelect(lesson)}
							>
								<div class="flex items-start justify-between gap-2">
									<div class="min-w-0 flex-1">
										<div class="flex items-center gap-1.5">
											<span class="font-mono text-[11px] text-muted-foreground">{lesson.id}</span>
											<Badge variant={statusVariant(lesson.status)} class="text-[10px] px-1 py-0">
												{lesson.status}
											</Badge>
										</div>
										<p class="mt-0.5 truncate text-xs font-medium text-muted-foreground">
											{lesson.title}
										</p>
									</div>
								</div>
							</button>
						{/each}
					</div>
				{/if}

				<!-- Resolved lessons -->
				{#if lessons.some((l) => l.status === "resolved")}
					<p class="mb-1.5 px-1 text-xs font-medium uppercase tracking-wide text-muted-foreground">
						Resolved
					</p>
					<div class="space-y-1">
						{#each lessons.filter((l) => l.status === "resolved") as lesson (lesson.id)}
							<button
								class="w-full rounded-md px-2 py-2 text-left opacity-60 transition-colors hover:bg-accent hover:opacity-100 {selectedId === lesson.id ? 'bg-accent opacity-100' : ''}"
								onclick={() => onSelect(lesson)}
							>
								<div class="flex items-center gap-1.5">
									<span class="font-mono text-[11px] text-muted-foreground">{lesson.id}</span>
									<p class="truncate text-xs text-muted-foreground">{lesson.title}</p>
								</div>
							</button>
						{/each}
					</div>
				{/if}
			{/if}
		</div>
	</ScrollArea>
</div>
