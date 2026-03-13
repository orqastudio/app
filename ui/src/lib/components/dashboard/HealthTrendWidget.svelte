<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import type { HealthSnapshot } from "$lib/types/artifact-graph";

	let snapshots = $state<HealthSnapshot[]>([]);
	let loading = $state(false);
	let loaded = $state(false);

	// Auto-load snapshots when the graph is ready
	$effect(() => {
		if (artifactGraphSDK.graph.size > 0 && !loaded && !loading) {
			void loadSnapshots();
		}
	});

	async function loadSnapshots() {
		loading = true;
		try {
			snapshots = await artifactGraphSDK.getHealthSnapshots(20);
			loaded = true;
		} catch {
			// Non-critical widget — silently degrade
		} finally {
			loading = false;
		}
	}

	// Reverse so oldest is first (for left-to-right sparkline)
	const chronological = $derived([...snapshots].reverse());

	interface SparklineConfig {
		label: string;
		key: keyof HealthSnapshot;
		color: string;
		strokeColor: string;
	}

	const sparklines: SparklineConfig[] = [
		{ label: "Errors", key: "error_count", color: "text-destructive", strokeColor: "#ef4444" },
		{ label: "Warnings", key: "warning_count", color: "text-warning", strokeColor: "#f59e0b" },
		{ label: "Orphans", key: "orphan_count", color: "text-muted-foreground", strokeColor: "#6b7280" },
		{ label: "Broken Refs", key: "broken_ref_count", color: "text-muted-foreground", strokeColor: "#6b7280" },
	];

	function sparklinePath(data: HealthSnapshot[], key: keyof HealthSnapshot, width: number, height: number): string {
		if (data.length < 2) return "";
		const values = data.map((s) => Number(s[key]));
		const max = Math.max(...values, 1); // At least 1 to avoid division by zero
		const stepX = width / (values.length - 1);
		const points = values.map((v, i) => `${i * stepX},${height - (v / max) * height}`);
		return `M${points.join(" L")}`;
	}

	function latestValue(key: keyof HealthSnapshot): number {
		if (snapshots.length === 0) return 0;
		return Number(snapshots[0][key]);
	}

	function trend(key: keyof HealthSnapshot): "up" | "down" | "flat" {
		if (snapshots.length < 2) return "flat";
		const current = Number(snapshots[0][key]);
		const previous = Number(snapshots[1][key]);
		if (current > previous) return "up";
		if (current < previous) return "down";
		return "flat";
	}

	function trendIndicator(key: keyof HealthSnapshot): string {
		const t = trend(key);
		// For error/warning metrics, "down" is good, "up" is bad
		if (t === "down") return "↓";
		if (t === "up") return "↑";
		return "—";
	}

	function trendColor(key: keyof HealthSnapshot): string {
		const t = trend(key);
		// For these metrics, lower is better
		if (t === "down") return "text-green-500";
		if (t === "up") return "text-destructive";
		return "text-muted-foreground";
	}
</script>

{#if loaded && snapshots.length >= 2}
	<Card.Root class="mb-4">
		<Card.Header class="pb-3">
			<Card.Title class="text-base">
				<div class="flex items-center gap-2">
					<TrendingUpIcon class="h-4 w-4 text-muted-foreground" />
					Health Trends
				</div>
			</Card.Title>
		</Card.Header>
		<Card.Content>
			{#if loading}
				<div class="flex items-center justify-center py-4">
					<LoadingSpinner />
				</div>
			{:else}
				<div class="grid grid-cols-2 gap-4">
					{#each sparklines as config (config.key)}
						<div class="flex items-center gap-3">
							<!-- Sparkline SVG -->
							<svg
								width="64"
								height="24"
								viewBox="0 0 64 24"
								class="shrink-0"
								fill="none"
								xmlns="http://www.w3.org/2000/svg"
							>
								<path
									d={sparklinePath(chronological, config.key, 64, 24)}
									stroke={config.strokeColor}
									stroke-width="1.5"
									stroke-linecap="round"
									stroke-linejoin="round"
									fill="none"
								/>
							</svg>
							<!-- Label + value -->
							<div class="min-w-0">
								<div class="text-xs text-muted-foreground">{config.label}</div>
								<div class="flex items-center gap-1">
									<span class="text-sm font-semibold tabular-nums {config.color}">
										{latestValue(config.key)}
									</span>
									<span class="text-xs {trendColor(config.key)}">
										{trendIndicator(config.key)}
									</span>
								</div>
							</div>
						</div>
					{/each}
				</div>
				<p class="mt-2 text-[10px] text-muted-foreground">
					Based on {snapshots.length} scan{snapshots.length !== 1 ? "s" : ""}
				</p>
			{/if}
		</Card.Content>
	</Card.Root>
{/if}
