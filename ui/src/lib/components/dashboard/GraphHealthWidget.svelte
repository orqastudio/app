<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
	import UnlinkIcon from "@lucide/svelte/icons/unlink";
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

	const errorCount = $derived(checks.filter((c) => c.severity === "Error").length);
	const warningCount = $derived(checks.filter((c) => c.severity === "Warning").length);
	const orphanCount = $derived(artifactGraphSDK.stats?.orphan_count ?? 0);
	const fixableCount = $derived(checks.filter((c) => c.auto_fixable).length);

	/** Health score: 100% minus weighted penalty for errors and warnings. */
	const healthScore = $derived.by(() => {
		if (!scanned) return null;
		const total = artifactGraphSDK.graph.size;
		if (total === 0) return 100;
		const penalty = errorCount * 3 + warningCount;
		const raw = Math.max(0, 100 - Math.round((penalty / Math.max(total, 1)) * 100));
		return Math.min(100, raw);
	});

	/** Traffic-light status based on error/warning thresholds. */
	type HealthStatus = "green" | "amber" | "red" | "unknown";
	const status = $derived.by((): HealthStatus => {
		if (!scanned) return "unknown";
		if (errorCount > 20 || warningCount > 30) return "red";
		if (errorCount >= 5 || warningCount >= 10) return "amber";
		return "green";
	});

	const circleClass = $derived.by(() => {
		if (status === "green") return "bg-green-500";
		if (status === "amber") return "bg-amber-500";
		if (status === "red") return "bg-destructive";
		return "bg-muted-foreground/30";
	});

	const scoreLabel = $derived.by(() => {
		if (!scanned) return "Not scanned";
		if (healthScore === null) return "—";
		return `${healthScore}%`;
	});
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
				<p class="text-xl font-semibold tabular-nums">
					{#if !scanned && loading}
						—
					{:else}
						{scoreLabel}
					{/if}
				</p>
				<p class="text-xs text-muted-foreground">
					{#if !scanned && loading}
						Scanning…
					{:else if status === "green"}
						No significant issues
					{:else if status === "amber"}
						Some issues found
					{:else if status === "red"}
						Needs attention
					{:else}
						Run a scan to assess
					{/if}
				</p>
			</div>
		</div>

		<!-- Breakdown -->
		{#if scanned}
			<div class="grid grid-cols-3 gap-2 text-center text-xs">
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
				<div class="flex flex-col items-center gap-1 rounded-md bg-muted/50 py-2">
					<UnlinkIcon class="h-3.5 w-3.5 {orphanCount > 0 ? 'text-warning' : 'text-muted-foreground'}" />
					<span class="{orphanCount > 0 ? 'text-warning font-semibold' : 'text-muted-foreground'} tabular-nums">
						{orphanCount}
					</span>
					<span class="text-muted-foreground">Orphan{orphanCount !== 1 ? "s" : ""}</span>
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
