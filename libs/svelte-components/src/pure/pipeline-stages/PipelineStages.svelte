<script lang="ts">
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "../tooltip/index.js";
	import { HStack } from "../layout/index.js";
	import type { Component } from "svelte";

	export type PipelineStage = {
		readonly key: string;
		readonly label: string;
		readonly count: number;
		readonly dotColorClass?: string;
		readonly icon?: Component;
		readonly borderClass?: string;
		readonly bgClass?: string;
		readonly iconClass?: string;
		readonly statusLabel?: string | null;
		readonly statusLabelClass?: string;
		readonly tooltipTitle?: string | null;
		readonly tooltipBody?: string | null;
	};

	export type PipelineEdge = {
		readonly count: number;
		readonly colorClass?: string;
	};

	let {
		stages,
		edges,
		onStageClick,
	}: {
		stages: readonly PipelineStage[];
		edges?: readonly PipelineEdge[];
		onStageClick?: (key: string) => void;
	} = $props();

	const hasEdges = $derived(edges !== undefined && edges.length === stages.length - 1);

	/**
	 * Resolve the Tailwind colour class for a pipeline edge connector.
	 * @param edge - The pipeline edge whose count and optional colorClass determine the colour
	 * @returns The colorClass if provided, otherwise a muted foreground class dimmed when count is zero
	 */
	function defaultColorClass(edge: PipelineEdge): string {
		return (
			edge.colorClass ?? (edge.count > 0 ? "text-muted-foreground" : "text-muted-foreground/30")
		);
	}
</script>

<HStack gap={2} align="stretch">
	{#each stages as stage, i (stage.key)}
		{#if stage.tooltipTitle}
			<TooltipRoot>
				<TooltipTrigger>
					{#snippet child({ props })}
						{#if onStageClick}
							<button
								{...props}
								class="hover:bg-accent/50 flex w-[88px] shrink-0 flex-col items-center gap-2 rounded-lg border px-2 py-3 transition-colors {stage.borderClass ??
									'border-border'} {stage.bgClass ?? 'bg-muted/30'}"
								onclick={() => onStageClick?.(stage.key)}
							>
								{@render stageInner(stage)}
							</button>
						{:else}
							<div
								{...props}
								class="flex w-[88px] shrink-0 flex-col items-center gap-2 rounded-lg border px-2 py-3 {stage.borderClass ??
									'border-border'} {stage.bgClass ?? 'bg-muted/30'}"
							>
								{@render stageInner(stage)}
							</div>
						{/if}
					{/snippet}
				</TooltipTrigger>
				<TooltipContent side="bottom">
					<p class="text-xs font-medium">{stage.tooltipTitle}</p>
					{#if stage.tooltipBody}
						<p class="text-muted-foreground mt-1 text-xs">{stage.tooltipBody}</p>
					{/if}
				</TooltipContent>
			</TooltipRoot>
		{:else if onStageClick}
			<TooltipRoot>
				<TooltipTrigger>
					{#snippet child({ props })}
						<button
							{...props}
							class="hover:bg-accent/50 flex w-[88px] shrink-0 flex-col items-center gap-2 rounded-lg border px-2 py-3 transition-colors {stage.borderClass ??
								'border-border'} {stage.bgClass ?? 'bg-muted/30'}"
							onclick={() => onStageClick?.(stage.key)}
						>
							{@render stageInner(stage)}
						</button>
					{/snippet}
				</TooltipTrigger>
				<TooltipContent side="bottom">
					<p class="text-xs">{stage.count} {stage.label.toLowerCase()}</p>
				</TooltipContent>
			</TooltipRoot>
		{:else}
			<div
				class="flex w-[88px] shrink-0 flex-col items-center gap-2 rounded-lg border px-2 py-3 {stage.borderClass ??
					'border-border'} {stage.bgClass ?? 'bg-muted/30'}"
			>
				{@render stageInner(stage)}
			</div>
		{/if}

		{#if i < stages.length - 1}
			{#if hasEdges && edges}
				<div
					class="flex min-w-0 flex-1 flex-col items-center justify-center gap-1 {defaultColorClass(
						edges[i],
					)}"
				>
					<svg
						class="h-4 w-full"
						viewBox="0 0 100 16"
						preserveAspectRatio="none"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
					>
						<line x1="0" y1="8" x2="88" y2="8" stroke="currentColor" stroke-width="1.5" />
						<polyline
							points="84,3 96,8 84,13"
							stroke="currentColor"
							stroke-width="1.5"
							fill="none"
						/>
					</svg>
					<span class="text-[10px] tabular-nums">{edges[i].count}</span>
				</div>
			{:else}
				<div class="text-muted-foreground/40 flex min-w-0 flex-1 items-center justify-center">
					<svg
						class="h-4 w-full"
						viewBox="0 0 100 16"
						preserveAspectRatio="none"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
					>
						<line x1="0" y1="8" x2="88" y2="8" stroke="currentColor" stroke-width="1.5" />
						<polyline
							points="84,3 96,8 84,13"
							stroke="currentColor"
							stroke-width="1.5"
							fill="none"
						/>
					</svg>
				</div>
			{/if}
		{/if}
	{/each}
</HStack>

{#snippet stageInner(stage: PipelineStage)}
	{#if stage.icon}
		<stage.icon class="h-5 w-5 {stage.iconClass ?? 'text-muted-foreground'}" />
	{:else if stage.dotColorClass}
		<span
			class="border-muted-foreground/40 flex h-4 w-4 items-center justify-center rounded-full border-2"
		>
			<span class="h-2 w-2 rounded-full {stage.dotColorClass}"></span>
		</span>
	{/if}
	<span class="text-foreground text-center text-[10px] leading-tight font-medium"
		>{stage.label}</span
	>
	<span class="text-foreground text-lg font-semibold tabular-nums">{stage.count}</span>
	{#if stage.statusLabel}
		<span class="text-[10px] font-medium {stage.statusLabelClass ?? 'text-muted-foreground'}">
			{stage.statusLabel}
		</span>
	{/if}
{/snippet}
