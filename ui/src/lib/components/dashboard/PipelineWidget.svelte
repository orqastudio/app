<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import EyeIcon from "@lucide/svelte/icons/eye";
	import BookOpenIcon from "@lucide/svelte/icons/book-open";
	import ScaleIcon from "@lucide/svelte/icons/scale";
	import WrenchIcon from "@lucide/svelte/icons/wrench";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import CheckCircle2Icon from "@lucide/svelte/icons/check-circle-2";
	import WorkflowIcon from "@lucide/svelte/icons/workflow";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import type { ArtifactNode, ArtifactRef } from "$lib/types/artifact-graph";
	import type { Component } from "svelte";

	// -------------------------------------------------------------------------
	// Pipeline stage definitions
	// -------------------------------------------------------------------------

	interface PipelineStage {
		key: string;
		label: string;
		/** Plural noun for display in reason text (e.g. "lessons", "decisions"). */
		artifactNoun: string;
		artifactType: string | null;
		icon: Component;
		/**
		 * Relationship types that indicate flow FROM this stage to the next.
		 * We include both directions of each pair so that we catch edges
		 * regardless of which end authored the relationship.
		 */
		outboundRelationships: string[];
	}

	const stages: PipelineStage[] = [
		{
			key: "observation",
			label: "Observation",
			artifactNoun: "lessons",
			artifactType: "lesson",
			icon: EyeIcon,
			outboundRelationships: [
				"informs",
				"informed-by",
				"grounded",
				"grounded-by",
			],
		},
		{
			key: "understanding",
			label: "Understanding",
			artifactNoun: "research docs",
			artifactType: "research",
			icon: BookOpenIcon,
			outboundRelationships: [
				"grounded",
				"grounded-by",
				"informs",
				"informed-by",
			],
		},
		{
			key: "principle",
			label: "Principle",
			artifactNoun: "decisions",
			artifactType: "decision",
			icon: ScaleIcon,
			outboundRelationships: [
				"practices",
				"practiced-by",
			],
		},
		{
			key: "practice",
			label: "Practice",
			artifactNoun: "skills",
			artifactType: "skill",
			icon: WrenchIcon,
			outboundRelationships: [
				"enforces",
				"enforced-by",
			],
		},
		{
			key: "enforcement",
			label: "Enforcement",
			artifactNoun: "rules",
			artifactType: "rule",
			icon: ShieldIcon,
			outboundRelationships: ["verifies", "verified-by"],
		},
		{
			key: "verification",
			label: "Verification",
			artifactNoun: "checks",
			artifactType: null,
			icon: CheckCircle2Icon,
			outboundRelationships: [],
		},
	];

	// -------------------------------------------------------------------------
	// Computed pipeline data — connectivity model
	// -------------------------------------------------------------------------

	interface StageData {
		stage: PipelineStage;
		artifacts: ArtifactNode[];
		count: number;
		/** How many artifacts have at least one relationship of ANY kind. */
		connectedCount: number;
		/** Ratio of connected to total (0-1). */
		connectivity: number;
		/** Connectivity status based on percentage with relationships. */
		status: "healthy" | "attention" | "isolated";
		/** Human-readable explanation for non-healthy status. */
		reason: string | null;
		/** Suggested action to improve connectivity. */
		suggestion: string | null;
	}

	interface EdgeData {
		fromKey: string;
		toKey: string;
		count: number;
	}

	/**
	 * Check whether a reference connects to an artifact in the target type.
	 */
	function refConnectsToType(ref: ArtifactRef, targetType: string | null): boolean {
		if (targetType === null) return false;
		const targetNode = artifactGraphSDK.resolve(ref.target_id);
		return targetNode?.artifact_type === targetType;
	}

	/**
	 * Count edges flowing between two adjacent stages based on relationship types.
	 */
	function countEdgesBetween(
		fromArtifacts: ArtifactNode[],
		toType: string | null,
		relationshipTypes: string[]
	): number {
		if (toType === null) return 0;
		let count = 0;
		for (const artifact of fromArtifacts) {
			for (const ref of artifact.references_out) {
				if (
					ref.relationship_type !== null &&
					relationshipTypes.includes(ref.relationship_type) &&
					refConnectsToType(ref, toType)
				) {
					count++;
				}
			}
		}
		return count;
	}

	/**
	 * Check if an artifact has ANY relationship (in or out).
	 */
	function hasAnyRelationship(artifact: ArtifactNode): boolean {
		return artifact.references_out.length > 0 || artifact.references_in.length > 0;
	}

	const stageDataList = $derived.by((): StageData[] => {
		return stages.map((stage) => {
			const artifacts =
				stage.artifactType !== null
					? artifactGraphSDK.byType(stage.artifactType)
					: [];
			const count = artifacts.length;

			// Connectivity: what % of artifacts have ANY relationship at all
			let connectedCount = 0;
			if (count > 0) {
				for (const artifact of artifacts) {
					if (hasAnyRelationship(artifact)) connectedCount++;
				}
			}

			const connectivity = count > 0 ? connectedCount / count : 1;

			let status: StageData["status"] = "healthy";
			let reason: string | null = null;
			let suggestion: string | null = null;

			if (count > 0) {
				const orphanCount = count - connectedCount;

				if (connectivity < 0.3) {
					status = "isolated";
					reason = `${orphanCount} of ${count} ${stage.artifactNoun} have no relationships`;
					suggestion = `Review orphaned ${stage.artifactNoun} and add cross-references`;
				} else if (connectivity < 0.7) {
					status = "attention";
					reason = `${orphanCount} of ${count} ${stage.artifactNoun} have no relationships`;
					suggestion = `Connect isolated ${stage.artifactNoun} to related artifacts`;
				}
			}

			return { stage, artifacts, count, connectedCount, connectivity, status, reason, suggestion };
		});
	});

	const edgeDataList = $derived.by((): EdgeData[] => {
		const edges: EdgeData[] = [];
		for (let i = 0; i < stages.length - 1; i++) {
			const fromStage = stages[i];
			const toStage = stages[i + 1];
			const fromArtifacts =
				fromStage.artifactType !== null
					? artifactGraphSDK.byType(fromStage.artifactType)
					: [];
			const count = countEdgesBetween(
				fromArtifacts,
				toStage.artifactType,
				fromStage.outboundRelationships
			);
			edges.push({
				fromKey: fromStage.key,
				toKey: toStage.key,
				count,
			});
		}
		return edges;
	});

	const hasData = $derived(artifactGraphSDK.graph.size > 0);

	// -------------------------------------------------------------------------
	// Visual helpers
	// -------------------------------------------------------------------------

	function statusBorderClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":
				return "border-red-400 dark:border-red-600";
			case "attention":
				return "border-amber-400 dark:border-amber-600";
			default:
				return "border-border";
		}
	}

	function statusBgClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":
				return "bg-red-50 dark:bg-red-950/30";
			case "attention":
				return "bg-amber-50 dark:bg-amber-950/30";
			default:
				return "bg-muted/30";
		}
	}

	function statusIconClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":
				return "text-red-500";
			case "attention":
				return "text-amber-500";
			default:
				return "text-muted-foreground";
		}
	}

	function statusLabel(data: StageData): string | null {
		if (data.count === 0) return null;
		const pct = Math.round(data.connectivity * 100);
		switch (data.status) {
			case "isolated":
				return `${pct}% connected`;
			case "attention":
				return `${pct}% connected`;
			default:
				return null;
		}
	}

	function statusLabelClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":
				return "text-red-500";
			case "attention":
				return "text-amber-500";
			default:
				return "text-muted-foreground";
		}
	}

	function edgeColorClass(count: number): string {
		return count > 0
			? "text-muted-foreground"
			: "text-muted-foreground/30";
	}
</script>

{#if hasData}
	<Card.Root>
		<Card.Header class="pb-3">
			<Card.Title class="text-base">
				<div class="flex items-center gap-2">
					<WorkflowIcon class="h-4 w-4 text-muted-foreground" />
					Knowledge Pipeline
				</div>
			</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="flex items-stretch gap-1 pb-2">
				{#each stageDataList as data, i (data.stage.key)}
					<!-- Stage box -->
					{#if data.reason !== null}
						<Tooltip.Root>
							<Tooltip.Trigger>
								{#snippet child({ props })}
									<div
										{...props}
										class="flex min-w-[80px] flex-1 flex-col items-center gap-1.5 rounded-lg border px-3 py-3 {statusBorderClass(data.status)} {statusBgClass(data.status)}"
									>
										<data.stage.icon
											class="h-5 w-5 {statusIconClass(data.status)}"
										/>
										<span class="text-xs font-medium text-foreground">
											{data.stage.label}
										</span>
										<span class="text-lg font-semibold tabular-nums text-foreground">
											{data.count}
										</span>
										{#if statusLabel(data) !== null}
											<span class="text-[10px] font-medium {statusLabelClass(data.status)}">
												{statusLabel(data)}
											</span>
										{/if}
									</div>
								{/snippet}
							</Tooltip.Trigger>
							<Tooltip.Content side="bottom" class="max-w-[240px]">
								<p class="text-xs font-medium">{data.reason}</p>
								{#if data.suggestion !== null}
									<p class="mt-1 text-xs text-muted-foreground">{data.suggestion}</p>
								{/if}
							</Tooltip.Content>
						</Tooltip.Root>
					{:else}
						<div
							class="flex min-w-[80px] flex-1 flex-col items-center gap-1.5 rounded-lg border px-3 py-3 {statusBorderClass(data.status)} {statusBgClass(data.status)}"
						>
							<data.stage.icon
								class="h-5 w-5 {statusIconClass(data.status)}"
							/>
							<span class="text-xs font-medium text-foreground">
								{data.stage.label}
							</span>
							<span class="text-lg font-semibold tabular-nums text-foreground">
								{data.count}
							</span>
						</div>
					{/if}

					<!-- Connecting arrow between stages -->
					{#if i < stageDataList.length - 1}
						<div class="flex flex-shrink-0 flex-col items-center justify-center gap-0.5 px-1">
							<svg
								width="32"
								height="16"
								viewBox="0 0 32 16"
								class={edgeColorClass(edgeDataList[i].count)}
								fill="none"
								xmlns="http://www.w3.org/2000/svg"
							>
								<line
									x1="0"
									y1="8"
									x2="24"
									y2="8"
									stroke="currentColor"
									stroke-width="1.5"
								/>
								<polyline
									points="20,4 26,8 20,12"
									stroke="currentColor"
									stroke-width="1.5"
									fill="none"
								/>
							</svg>
							<span
								class="text-[10px] tabular-nums {edgeColorClass(edgeDataList[i].count)}"
							>
								{edgeDataList[i].count}
							</span>
						</div>
					{/if}
				{/each}
			</div>

			<!-- Legend -->
			<div class="mt-3 flex items-center gap-4 text-[10px] text-muted-foreground">
				<span class="flex items-center gap-1">
					<span class="inline-block h-2 w-2 rounded-full bg-red-400"></span>
					Isolated (&lt;30% connected)
				</span>
				<span class="flex items-center gap-1">
					<span class="inline-block h-2 w-2 rounded-full bg-amber-400"></span>
					Attention (30-70% connected)
				</span>
			</div>
		</Card.Content>
	</Card.Root>
{/if}
