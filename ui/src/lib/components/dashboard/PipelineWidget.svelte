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
			// Lessons flow forward via: informs/informed-by (to research),
			// grounded/grounded-by (to decisions/research)
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
			// Research flows forward via: grounded/grounded-by (to decisions)
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
			// Decisions flow forward via: practiced-by/practices (to skills)
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
			// Skills flow forward via: enforces/enforced-by (to rules)
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
			// Rules flow forward via: verifies/verified-by (to verification)
			// Terminal stage for now — verification has no artifact type.
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
	// Computed pipeline data
	// -------------------------------------------------------------------------

	interface StageData {
		stage: PipelineStage;
		artifacts: ArtifactNode[];
		count: number;
		/** How many artifacts have at least one outgoing edge to the next stage. */
		connectedCount: number;
		/** Ratio of connected to total (0-1). */
		flowRate: number;
		/** Bottleneck status based on flow rate. */
		status: "healthy" | "bottleneck" | "stuck";
		/** Human-readable reason for stuck/bottleneck status. */
		reason: string | null;
		/** Suggested action to improve flow. */
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

	/** Map from next-stage artifact type to a human-readable action target. */
	const NEXT_STAGE_LABELS: Record<string, string> = {
		research: "research",
		decision: "decisions",
		skill: "skills",
		rule: "rules",
	};

	function buildSuggestion(
		stageName: string,
		artifactNoun: string,
		nextType: string | null,
	): string {
		const target = nextType !== null ? NEXT_STAGE_LABELS[nextType] ?? nextType : "the next stage";
		return `Review unlinked ${artifactNoun} and connect them to ${target}`;
	}

	const stageDataList = $derived.by((): StageData[] => {
		return stages.map((stage, index) => {
			const artifacts =
				stage.artifactType !== null
					? artifactGraphSDK.byType(stage.artifactType)
					: [];
			const count = artifacts.length;

			// For the last stage (Verification) or stages with no artifacts,
			// flow rate is not applicable
			const isLastStage = index === stages.length - 1;
			const nextStage = isLastStage ? null : stages[index + 1];
			const nextType = nextStage?.artifactType ?? null;

			let connectedCount = 0;
			if (!isLastStage && count > 0) {
				for (const artifact of artifacts) {
					const hasDownstream = artifact.references_out.some(
						(ref: ArtifactRef) =>
							ref.relationship_type !== null &&
							stage.outboundRelationships.includes(ref.relationship_type) &&
							refConnectsToType(ref, nextType)
					);
					if (hasDownstream) connectedCount++;
				}
			}

			const flowRate = count > 0 ? connectedCount / count : 1;

			let status: StageData["status"] = "healthy";
			let reason: string | null = null;
			let suggestion: string | null = null;

			if (!isLastStage && count > 0) {
				const unlinkedCount = count - connectedCount;

				if (flowRate === 0) {
					status = "stuck";
					reason = `${unlinkedCount} of ${count} ${stage.artifactNoun} have no downstream link`;
					suggestion = buildSuggestion(stage.key, stage.artifactNoun, nextType);
				} else if (flowRate < 0.3) {
					status = "bottleneck";
					reason = `${unlinkedCount} of ${count} ${stage.artifactNoun} have no downstream link`;
					suggestion = buildSuggestion(stage.key, stage.artifactNoun, nextType);
				}
			}

			return { stage, artifacts, count, connectedCount, flowRate, status, reason, suggestion };
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
			case "stuck":
				return "border-red-400 dark:border-red-600";
			case "bottleneck":
				return "border-amber-400 dark:border-amber-600";
			default:
				return "border-border";
		}
	}

	function statusBgClass(status: StageData["status"]): string {
		switch (status) {
			case "stuck":
				return "bg-red-50 dark:bg-red-950/30";
			case "bottleneck":
				return "bg-amber-50 dark:bg-amber-950/30";
			default:
				return "bg-muted/30";
		}
	}

	function statusIconClass(status: StageData["status"]): string {
		switch (status) {
			case "stuck":
				return "text-red-500";
			case "bottleneck":
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
										{#if data.status === "stuck"}
											<span class="text-[10px] font-medium text-red-500">
												stuck
											</span>
										{:else if data.status === "bottleneck"}
											<span class="text-[10px] font-medium text-amber-500">
												bottleneck
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
					Stuck (0% flow)
				</span>
				<span class="flex items-center gap-1">
					<span class="inline-block h-2 w-2 rounded-full bg-amber-400"></span>
					Bottleneck (&lt;30% flow)
				</span>
			</div>
		</Card.Content>
	</Card.Root>
{/if}
