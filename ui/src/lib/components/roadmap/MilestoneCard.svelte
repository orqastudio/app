<script lang="ts">
	import type { ArtifactNode } from "$lib/types/artifact-graph";
	import StatusIndicator from "$lib/components/shared/StatusIndicator.svelte";
	import SmallBadge from "$lib/components/shared/SmallBadge.svelte";

	let {
		milestone,
		epicCount,
		doneEpicCount,
		inProgressEpics,
		criticalEpics,
		onClick,
	}: {
		milestone: ArtifactNode;
		epicCount: number;
		doneEpicCount: number;
		inProgressEpics: ArtifactNode[];
		criticalEpics: ArtifactNode[];
		onClick: () => void;
	} = $props();

	const progressPct = $derived(
		epicCount > 0 ? (doneEpicCount / epicCount) * 100 : 0,
	);
</script>

<button
	class="group w-full rounded-xl border border-border bg-card p-4 text-left transition-all hover:border-border/80 hover:bg-accent/40 hover:shadow-sm"
	onclick={onClick}
>
	<!-- Header -->
	<div class="flex items-start justify-between gap-3">
		<div class="flex min-w-0 flex-col gap-1">
			<span class="truncate text-sm font-semibold leading-tight">{milestone.title}</span>
			{#if milestone.description}
				<p class="line-clamp-2 text-xs text-muted-foreground">{milestone.description}</p>
			{/if}
		</div>
		<div class="shrink-0">
			<StatusIndicator status={milestone.status ?? "planning"} mode="badge" />
		</div>
	</div>

	<!-- Progress -->
	{#if epicCount > 0}
		<div class="mt-3">
			<div class="mb-1 flex items-center justify-between">
				<span class="text-[10px] text-muted-foreground uppercase tracking-wide">Progress</span>
				<span class="text-[10px] tabular-nums text-muted-foreground">
					{doneEpicCount}/{epicCount} epics
				</span>
			</div>
			<div class="h-1.5 rounded-full bg-muted">
				<div
					class="h-1.5 rounded-full bg-emerald-500 transition-all duration-300"
					style="width: {progressPct}%"
				></div>
			</div>
		</div>
	{:else}
		<p class="mt-3 text-[10px] text-muted-foreground">No epics yet</p>
	{/if}

	<!-- In-progress epics -->
	{#if inProgressEpics.length > 0}
		<div class="mt-3 border-t border-border/50 pt-2">
			<p class="mb-1 text-[10px] uppercase tracking-wide text-muted-foreground">Now</p>
			<div class="flex flex-col gap-1">
				{#each inProgressEpics.slice(0, 2) as epic (epic.id)}
					<div class="flex items-center gap-1.5 text-xs">
						<span class="h-1.5 w-1.5 shrink-0 rounded-full bg-blue-500"></span>
						<span class="truncate text-muted-foreground">{epic.title}</span>
					</div>
				{/each}
				{#if inProgressEpics.length > 2}
					<span class="text-[10px] text-muted-foreground/60">
						+{inProgressEpics.length - 2} more
					</span>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Critical P1 epics not done -->
	{#if criticalEpics.length > 0}
		<div class="mt-2 border-t border-border/50 pt-2">
			<p class="mb-1 text-[10px] uppercase tracking-wide text-muted-foreground">Critical</p>
			<div class="flex flex-col gap-1">
				{#each criticalEpics.slice(0, 2) as epic (epic.id)}
					<div class="flex items-center gap-1.5 text-xs">
						<SmallBadge variant="destructive">P1</SmallBadge>
						<span class="truncate text-muted-foreground">{epic.title}</span>
					</div>
				{/each}
				{#if criticalEpics.length > 2}
					<span class="text-[10px] text-muted-foreground/60">
						+{criticalEpics.length - 2} more
					</span>
				{/if}
			</div>
		</div>
	{/if}
</button>
