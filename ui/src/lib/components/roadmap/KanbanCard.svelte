<script lang="ts">
	import type { ArtifactNode } from "$lib/types/artifact-graph";
	import StatusIndicator from "$lib/components/shared/StatusIndicator.svelte";
	import SmallBadge from "$lib/components/shared/SmallBadge.svelte";
	import type { BadgeVariant } from "$lib/components/ui/badge";

	let {
		node,
		taskCount,
		onClick,
		onDragStart,
	}: {
		node: ArtifactNode;
		taskCount?: { done: number; total: number };
		onClick?: () => void;
		onDragStart?: (e: DragEvent) => void;
	} = $props();

	function priorityVariant(priority: string | null): BadgeVariant {
		if (priority === "P1") return "destructive";
		if (priority === "P2") return "default";
		return "secondary";
	}

	const progressPct = $derived(
		taskCount && taskCount.total > 0
			? (taskCount.done / taskCount.total) * 100
			: 0,
	);
</script>

{#snippet cardContent()}
	<!-- Title row -->
	<div class="flex items-start justify-between gap-2">
		<div class="flex min-w-0 items-center gap-2">
			<StatusIndicator status={node.status ?? "captured"} mode="dot" />
			<span class="truncate text-sm font-medium">{node.title}</span>
		</div>
		{#if node.priority}
			<SmallBadge variant={priorityVariant(node.priority)}>
				{node.priority}
			</SmallBadge>
		{/if}
	</div>

	<!-- Description -->
	{#if node.description}
		<p class="mt-1.5 line-clamp-2 text-xs text-muted-foreground">
			{node.description}
		</p>
	{/if}

	<!-- Task progress bar -->
	{#if taskCount && taskCount.total > 0}
		<div class="mt-2 flex items-center gap-2">
			<div class="h-1.5 flex-1 rounded-full bg-muted">
				<div
					class="h-1.5 rounded-full bg-emerald-500 transition-all"
					style="width: {progressPct}%"
				></div>
			</div>
			<span class="shrink-0 text-[10px] tabular-nums text-muted-foreground">
				{taskCount.done}/{taskCount.total}
			</span>
		</div>
	{/if}

	<!-- ID chip -->
	<div class="mt-1.5">
		<span class="text-[10px] font-mono text-muted-foreground/60">{node.id}</span>
	</div>
{/snippet}

{#if onClick}
	<button
		class="w-full rounded-lg border border-border bg-card p-3 text-left transition-colors hover:bg-accent/50 hover:border-border/80"
		draggable={onDragStart !== undefined}
		ondragstart={onDragStart}
		onclick={onClick}
	>
		{@render cardContent()}
	</button>
{:else}
	<div
		class="w-full rounded-lg border border-border bg-card p-3 cursor-grab active:cursor-grabbing"
		draggable={onDragStart !== undefined}
		ondragstart={onDragStart}
		role="listitem"
	>
		{@render cardContent()}
	</div>
{/if}
