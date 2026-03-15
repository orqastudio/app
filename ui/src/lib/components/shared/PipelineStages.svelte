<script lang="ts">
	import * as Tooltip from "$lib/components/ui/tooltip";
	import type { Component } from "svelte";

	// -------------------------------------------------------------------------
	// Types
	// -------------------------------------------------------------------------

	export type PipelineStage = {
		key: string;
		label: string;
		count: number;
		/** Tailwind color class for the inner dot of the radio-button indicator (e.g. "bg-blue-500"). */
		dotColorClass?: string;
		/** Icon component rendered above the label (takes precedence over dotColorClass). */
		icon?: Component;
		/** Extra CSS classes applied to the stage pill border. */
		borderClass?: string;
		/** Extra CSS classes applied to the stage pill background. */
		bgClass?: string;
		/** Extra CSS classes applied to the icon. */
		iconClass?: string;
		/** Small status label rendered below count (e.g. "72% connected"). */
		statusLabel?: string | null;
		/** Extra CSS classes applied to statusLabel. */
		statusLabelClass?: string;
		/** Tooltip content — first line. Enables the tooltip when set. */
		tooltipTitle?: string | null;
		/** Tooltip content — second line, muted. */
		tooltipBody?: string | null;
	};

	export type PipelineEdge = {
		/** Count of connections flowing between adjacent stages. */
		count: number;
		/** Tailwind color class for the arrow/connector (defaults to count > 0 colour). */
		colorClass?: string;
	};

	// -------------------------------------------------------------------------
	// Props
	// -------------------------------------------------------------------------

	let {
		stages,
		edges,
		onStageClick,
	}: {
		stages: PipelineStage[];
		/**
		 * Edge data between adjacent stages. Length must equal stages.length - 1.
		 * Omit entirely (or pass an empty array) to render simple chevron connectors
		 * without counts.
		 */
		edges?: PipelineEdge[];
		onStageClick?: (key: string) => void;
	} = $props();

	// -------------------------------------------------------------------------
	// Helpers
	// -------------------------------------------------------------------------

	/** True when edges are provided and have the expected length. */
	const hasEdges = $derived(
		edges !== undefined && edges.length === stages.length - 1
	);

	function defaultColorClass(edge: PipelineEdge): string {
		return edge.colorClass ?? (edge.count > 0 ? "text-muted-foreground" : "text-muted-foreground/30");
	}
</script>

<!--
	Layout: stages have a fixed width; arrows (flex-1) fill the remaining space.
	This keeps all stage pills the same size regardless of label length, and lets
	the arrows stretch proportionally between them.
-->
<div class="flex items-stretch">
	{#each stages as stage, i (stage.key)}
		<!-- ------------------------------------------------------------------ -->
		<!-- Stage pill — fixed width, wrapped in Tooltip when tooltipTitle set  -->
		<!-- ------------------------------------------------------------------ -->
		{#if stage.tooltipTitle}
			<Tooltip.Root>
				<Tooltip.Trigger>
					{#snippet child({ props })}
						{#if onStageClick}
							<button
								{...props}
								class="flex w-[72px] shrink-0 flex-col items-center gap-1.5 rounded-lg border px-2 py-3 transition-colors hover:bg-accent/50 {stage.borderClass ?? 'border-border'} {stage.bgClass ?? 'bg-muted/30'}"
								onclick={() => onStageClick?.(stage.key)}
							>
								{@render stageInner(stage)}
							</button>
						{:else}
							<div
								{...props}
								class="flex w-[72px] shrink-0 flex-col items-center gap-1.5 rounded-lg border px-2 py-3 {stage.borderClass ?? 'border-border'} {stage.bgClass ?? 'bg-muted/30'}"
							>
								{@render stageInner(stage)}
							</div>
						{/if}
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom" class="max-w-[240px]">
					<p class="text-xs font-medium">{stage.tooltipTitle}</p>
					{#if stage.tooltipBody}
						<p class="mt-1 text-xs text-muted-foreground">{stage.tooltipBody}</p>
					{/if}
				</Tooltip.Content>
			</Tooltip.Root>
		{:else if onStageClick}
			<Tooltip.Root>
				<Tooltip.Trigger>
					{#snippet child({ props })}
						<button
							{...props}
							class="flex w-[72px] shrink-0 flex-col items-center gap-1.5 rounded-lg border px-2 py-3 transition-colors hover:bg-accent/50 {stage.borderClass ?? 'border-border'} {stage.bgClass ?? 'bg-muted/30'}"
							onclick={() => onStageClick?.(stage.key)}
						>
							{@render stageInner(stage)}
						</button>
					{/snippet}
				</Tooltip.Trigger>
				<Tooltip.Content side="bottom">
					<p class="text-xs">{stage.count} {stage.label.toLowerCase()}</p>
				</Tooltip.Content>
			</Tooltip.Root>
		{:else}
			<div
				class="flex w-[72px] shrink-0 flex-col items-center gap-1.5 rounded-lg border px-2 py-3 {stage.borderClass ?? 'border-border'} {stage.bgClass ?? 'bg-muted/30'}"
			>
				{@render stageInner(stage)}
			</div>
		{/if}

		<!-- ------------------------------------------------------------------ -->
		<!-- Connector between stages — flex-1 so it fills available space      -->
		<!-- ------------------------------------------------------------------ -->
		{#if i < stages.length - 1}
			{#if hasEdges && edges}
				<!-- Rich arrow connector with edge count — stretches between pills -->
				<div class="flex min-w-0 flex-1 flex-col items-center justify-center gap-0.5">
					<svg
						class="w-full max-w-[48px] {defaultColorClass(edges[i])}"
						height="16"
						viewBox="0 0 48 16"
						preserveAspectRatio="xMidYMid meet"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
					>
						<line x1="0" y1="8" x2="38" y2="8" stroke="currentColor" stroke-width="1.5" />
						<polyline points="34,4 42,8 34,12" stroke="currentColor" stroke-width="1.5" fill="none" />
					</svg>
					<span class="text-[10px] tabular-nums {defaultColorClass(edges[i])}">
						{edges[i].count}
					</span>
				</div>
			{:else}
				<!-- Simple chevron connector — stretches between pills -->
				<div class="flex min-w-0 flex-1 items-center justify-center text-muted-foreground/40">
					<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
						<polyline points="4,4 10,8 4,12" stroke="currentColor" stroke-width="1.5" fill="none" />
					</svg>
				</div>
			{/if}
		{/if}
	{/each}
</div>

<!-- -----------------------------------------------------------------------  -->
<!-- Inner stage content snippet                                               -->
<!-- ----------------------------------------------------------------------- -->
{#snippet stageInner(stage: PipelineStage)}
	{#if stage.icon}
		<stage.icon class="h-5 w-5 {stage.iconClass ?? 'text-muted-foreground'}" />
	{:else if stage.dotColorClass}
		<!--
			Radio-button style: outer ring + inner filled dot.
			The outer ring is always border-current (inherits from the pill context),
			the inner dot uses dotColorClass for the status colour.
		-->
		<span class="flex h-4 w-4 items-center justify-center rounded-full border-2 border-muted-foreground/40">
			<span class="h-2 w-2 rounded-full {stage.dotColorClass}"></span>
		</span>
	{/if}
	<span class="text-[10px] font-medium text-foreground leading-tight text-center">{stage.label}</span>
	<span class="text-lg font-semibold tabular-nums text-foreground">{stage.count}</span>
	{#if stage.statusLabel}
		<span class="text-[10px] font-medium {stage.statusLabelClass ?? 'text-muted-foreground'}">
			{stage.statusLabel}
		</span>
	{/if}
{/snippet}
