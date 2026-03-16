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
	import EyeIcon from "@lucide/svelte/icons/eye";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import { getGraphViz } from "$lib/graph-viz.svelte";
	import type { IntegrityCheck } from "@orqastudio/types";

	const graphViz = getGraphViz();

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
	const health = $derived(graphViz.graphHealth);

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

	// Orphan severity: green <5%, amber 5-15%, red >15%
	const orphanSeverity = $derived.by(() => {
		if (health.orphanPercentage > 15) return "text-destructive";
		if (health.orphanPercentage > 5) return "text-warning";
		return "text-emerald-500";
	});

	// Avg degree: green >=4, cyan 3-4, amber 2-3, red <2
	const degreeSeverity = $derived.by(() => {
		if (health.avgDegree >= 4) return "text-emerald-500";
		if (health.avgDegree >= 3) return "text-cyan-500";
		if (health.avgDegree >= 2) return "text-warning";
		return "text-destructive";
	});
</script>

<Card.Root class="gap-2 flex flex-col h-full">
	<Card.Header class="pb-2">
		<Card.Title class="flex items-center gap-1.5 text-sm font-semibold">
			<EyeIcon class="h-4 w-4 text-muted-foreground" />
			Clarity
		</Card.Title>
		<Card.Description class="text-xs">Where You Are</Card.Description>
		<Card.Action>
			{#if loading}
				<LoadingSpinner size="sm" />
			{:else}
				<div class="flex items-center gap-2">
					<span class="text-sm font-semibold tabular-nums">{scoreLabel}</span>
					<span class="relative flex h-3 w-3 shrink-0 items-center justify-center">
						<span class="absolute h-3 w-3 rounded-full {circleClass} opacity-30"></span>
						<span class="h-1.5 w-1.5 rounded-full {circleClass}"></span>
					</span>
				</div>
			{/if}
		</Card.Action>
	</Card.Header>
	<Card.Content class="flex flex-1 flex-col gap-3 pt-0">
		{#if health.totalNodes > 0}
			<div class="grid grid-cols-2 gap-2 flex-1 text-center text-xs">
				<!-- Clusters -->
				<Tooltip.Root delayDuration={300}>
					<Tooltip.Trigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<NetworkIcon class="h-3.5 w-3.5 {health.componentCount > 1 ? 'text-warning' : 'text-muted-foreground'}" />
						<span class="{health.componentCount > 1 ? 'text-warning font-semibold' : 'text-muted-foreground'} tabular-nums">
							{health.componentCount}
						</span>
						<span class="text-muted-foreground">Cluster{health.componentCount !== 1 ? "s" : ""}</span>
					</Tooltip.Trigger>
					<Tooltip.Content side="bottom" class="w-64 text-xs">
						<p class="font-medium mb-1">Connected Components</p>
						<p class="text-muted-foreground">The number of disconnected subgraphs. A healthy graph has 1 cluster — all artifacts are reachable from each other. Multiple clusters indicate orphaned groups of artifacts that aren't connected to the main knowledge graph.</p>
					</Tooltip.Content>
				</Tooltip.Root>

				<!-- Orphans -->
				<Tooltip.Root delayDuration={300}>
					<Tooltip.Trigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<UnlinkIcon class="h-3.5 w-3.5 {orphanSeverity}" />
						<span class="{orphanSeverity} font-semibold tabular-nums">
							{health.orphanCount} <span class="font-normal">({health.orphanPercentage}%)</span>
						</span>
						<span class="text-muted-foreground">Orphans</span>
					</Tooltip.Trigger>
					<Tooltip.Content side="bottom" class="w-64 text-xs">
						<p class="font-medium mb-1">Orphaned Artifacts</p>
						<p class="text-muted-foreground">Artifacts with no incoming references — nothing points to them. They exist in isolation and won't be discovered through graph traversal. Each orphan should either be connected via relationships or removed if no longer relevant.</p>
					</Tooltip.Content>
				</Tooltip.Root>

				<!-- Avg Degree -->
				<Tooltip.Root delayDuration={300}>
					<Tooltip.Trigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<GitBranchIcon class="h-3.5 w-3.5 {degreeSeverity}" />
						<span class="{degreeSeverity} font-semibold tabular-nums">{health.avgDegree}</span>
						<span class="text-muted-foreground">Avg Degree</span>
					</Tooltip.Trigger>
					<Tooltip.Content side="bottom" class="w-64 text-xs">
						<p class="font-medium mb-1">Average Connection Degree</p>
						<p class="text-muted-foreground">The average number of relationships per artifact. Higher means a more interconnected knowledge graph. A well-connected graph has an average degree of 4+ — each artifact relates to multiple others.</p>
					</Tooltip.Content>
				</Tooltip.Root>

				<!-- Scan Results -->
				{#if scanned}
					<Tooltip.Root delayDuration={300}>
						<Tooltip.Trigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
							{#if errorCount > 0}
								<CircleAlertIcon class="h-3.5 w-3.5 text-destructive" />
								<span class="text-destructive font-semibold tabular-nums">{errorCount}E / {warningCount}W</span>
							{:else if warningCount > 0}
								<TriangleAlertIcon class="h-3.5 w-3.5 text-warning" />
								<span class="text-warning font-semibold tabular-nums">{warningCount}W</span>
							{:else}
								<CircleAlertIcon class="h-3.5 w-3.5 text-emerald-500" />
								<span class="text-emerald-500 font-semibold">Clean</span>
							{/if}
							<span class="text-muted-foreground">Integrity</span>
						</Tooltip.Trigger>
						<Tooltip.Content side="bottom" class="w-64 text-xs">
							<p class="font-medium mb-1">Integrity Scan Results</p>
							<p class="text-muted-foreground">File-level checks: broken references, invalid statuses, missing required fields, schema violations. Errors must be fixed. Warnings indicate potential issues. Use Auto-fix for machine-fixable problems.</p>
						</Tooltip.Content>
					</Tooltip.Root>
				{:else}
					<div class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 text-muted-foreground">
						<ScanIcon class="h-3.5 w-3.5" />
						<span class="tabular-nums">—</span>
						<span>Integrity</span>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Actions -->
		<div class="grid grid-cols-2 gap-2 mt-auto">
			<Button variant="outline" size="sm" onclick={onScan} disabled={loading || fixing}>
				{#if loading}
					<span class="mr-2"><LoadingSpinner size="sm" /></span>
				{:else}
					<ScanIcon class="mr-2 h-3.5 w-3.5" />
				{/if}
				Scan
			</Button>
			<Button variant="outline" size="sm" onclick={onAutoFix} disabled={loading || fixing || !scanned || fixableCount === 0 || !onAutoFix}>
				{#if fixing}
					<span class="mr-2"><LoadingSpinner size="sm" /></span>
				{:else}
					<WrenchIcon class="mr-1.5 h-3.5 w-3.5" />
				{/if}
				Auto-fix{scanned && fixableCount > 0 ? ` (${fixableCount})` : ""}
			</Button>
		</div>
	</Card.Content>
</Card.Root>
