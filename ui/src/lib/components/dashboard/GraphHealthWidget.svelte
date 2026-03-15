<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
	import UnlinkIcon from "@lucide/svelte/icons/unlink";
	import NetworkIcon from "@lucide/svelte/icons/network";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import ScanIcon from "@lucide/svelte/icons/scan";
	import WrenchIcon from "@lucide/svelte/icons/wrench";
	import ActivityIcon from "@lucide/svelte/icons/activity";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import type { IntegrityCheck } from "$lib/types/artifact-graph";

	interface Props {
		checks: IntegrityCheck[];
		loading: boolean;
		fixing?: boolean;
		scanned: boolean;
		onScan: () => void;
		onAutoFix?: () => void;
	}

	const { checks, loading, fixing = false, scanned, onScan, onAutoFix }: Props = $props();

	// Graph-theoretic metrics — reactive, no scan needed.
	const health = $derived(artifactGraphSDK.graphHealth);

	// Score: percentage of graph in the largest connected component.
	const healthScore = $derived(Math.round(health.largestComponentRatio * 100));

	// Traffic light based on largestComponentRatio thresholds.
	type HealthStatus = "green" | "amber" | "red" | "empty";
	const status = $derived.by((): HealthStatus => {
		if (health.totalNodes === 0) return "empty";
		if (health.largestComponentRatio > 0.9) return "green";
		if (health.largestComponentRatio > 0.7) return "amber";
		return "red";
	});

	const circleClass = $derived.by(() => {
		if (status === "green") return "bg-green-500";
		if (status === "amber") return "bg-amber-500";
		if (status === "red") return "bg-destructive";
		return "bg-muted-foreground/30";
	});

	const scoreLabel = $derived.by(() => {
		if (health.totalNodes === 0) return "—";
		return `${healthScore}%`;
	});

	// Integrity scan counters (complementary to graph metrics).
	const errorCount = $derived(checks.filter((c) => c.severity === "Error").length);
	const warningCount = $derived(checks.filter((c) => c.severity === "Warning").length);
	const fixableCount = $derived(checks.filter((c) => c.auto_fixable).length);
</script>

<Card.Root>
	<Card.Header class="pb-2">
		<div class="flex items-center justify-between">
			<Card.Title class="flex items-center gap-2 text-sm">
				<ActivityIcon class="h-4 w-4 text-muted-foreground" />
				Graph Health
			</Card.Title>
			{#if loading}
				<LoadingSpinner size="sm" />
			{/if}
		</div>
	</Card.Header>
	<Card.Content class="flex flex-col gap-4">
		<!-- Status circle + score -->
		<div class="flex items-center gap-4">
			<div class="relative flex h-12 w-12 shrink-0 items-center justify-center">
				<span class="h-12 w-12 rounded-full {circleClass} opacity-20"></span>
				<span class="absolute h-8 w-8 rounded-full {circleClass}"></span>
			</div>
			<div>
				<p class="text-xl font-semibold tabular-nums">{scoreLabel}</p>
				<p class="text-xs text-muted-foreground">
					{#if status === "empty"}
						No graph data yet
					{:else if status === "green"}
						Well connected
					{:else if status === "amber"}
						Fragmented — some clusters
					{:else}
						Highly fragmented
					{/if}
				</p>
			</div>
		</div>

		<!-- Graph-theoretic metrics (always visible once graph has nodes) -->
		{#if health.totalNodes > 0}
			<div class="grid grid-cols-4 gap-2 text-center text-xs">
				<div class="flex flex-col items-center gap-1 rounded-md bg-muted/50 py-2">
					<NetworkIcon class="h-3.5 w-3.5 {health.componentCount > 1 ? 'text-warning' : 'text-muted-foreground'}" />
					<span class="{health.componentCount > 1 ? 'text-warning font-semibold' : 'text-muted-foreground'} tabular-nums">
						{health.componentCount}
					</span>
					<span class="text-muted-foreground">Cluster{health.componentCount !== 1 ? "s" : ""}</span>
				</div>
				<div class="flex flex-col items-center gap-1 rounded-md bg-muted/50 py-2">
					<UnlinkIcon class="h-3.5 w-3.5 {health.orphanCount > 0 ? 'text-warning' : 'text-muted-foreground'}" />
					<span class="{health.orphanCount > 0 ? 'text-warning font-semibold' : 'text-muted-foreground'} tabular-nums">
						{health.orphanCount}
					</span>
					<span class="text-muted-foreground">Orphan{health.orphanCount !== 1 ? "s" : ""}</span>
				</div>
				<div class="flex flex-col items-center gap-1 rounded-md bg-muted/50 py-2">
					<UnlinkIcon class="h-3.5 w-3.5 text-muted-foreground" />
					<span class="tabular-nums text-muted-foreground">{health.orphanPercentage}%</span>
					<span class="text-muted-foreground">Orphan %</span>
				</div>
				<div class="flex flex-col items-center gap-1 rounded-md bg-muted/50 py-2">
					<GitBranchIcon class="h-3.5 w-3.5 text-muted-foreground" />
					<span class="tabular-nums text-muted-foreground">{health.avgDegree}</span>
					<span class="text-muted-foreground">Avg degree</span>
				</div>
			</div>
		{/if}

		<!-- Integrity scan results (shown only after a scan) -->
		{#if scanned}
			<div class="grid grid-cols-2 gap-2 text-center text-xs">
				<div class="flex flex-col items-center gap-1 rounded-md bg-muted/50 py-2">
					<CircleAlertIcon class="h-3.5 w-3.5 {errorCount > 0 ? 'text-destructive' : 'text-muted-foreground'}" />
					<span class="{errorCount > 0 ? 'text-destructive font-semibold' : 'text-muted-foreground'} tabular-nums">
						{errorCount}
					</span>
					<span class="text-muted-foreground">Error{errorCount !== 1 ? "s" : ""}</span>
				</div>
				<div class="flex flex-col items-center gap-1 rounded-md bg-muted/50 py-2">
					<TriangleAlertIcon class="h-3.5 w-3.5 {warningCount > 0 ? 'text-warning' : 'text-muted-foreground'}" />
					<span class="{warningCount > 0 ? 'text-warning font-semibold' : 'text-muted-foreground'} tabular-nums">
						{warningCount}
					</span>
					<span class="text-muted-foreground">Warning{warningCount !== 1 ? "s" : ""}</span>
				</div>
			</div>
		{/if}

		<!-- Actions -->
		<div class="flex gap-2">
			<Button
				variant="outline"
				size="sm"
				onclick={onScan}
				disabled={loading || fixing}
				class="flex-1"
			>
				{#if loading}
					<span class="mr-2"><LoadingSpinner size="sm" /></span>
				{:else}
					<ScanIcon class="mr-2 h-3.5 w-3.5" />
				{/if}
				Run integrity scan
			</Button>
			{#if scanned && fixableCount > 0 && onAutoFix}
				<Button
					variant="outline"
					size="sm"
					onclick={onAutoFix}
					disabled={loading || fixing}
				>
					{#if fixing}
						<span class="mr-2"><LoadingSpinner size="sm" /></span>
					{:else}
						<WrenchIcon class="mr-1.5 h-3.5 w-3.5" />
					{/if}
					Auto-fix ({fixableCount})
				</Button>
			{/if}
		</div>
	</Card.Content>
</Card.Root>
