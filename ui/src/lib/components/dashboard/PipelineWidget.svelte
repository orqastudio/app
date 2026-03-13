<script lang="ts">
	import * as Card from "$lib/components/ui/card";
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
		artifactType: string | null;
		icon: Component;
		/** Relationship types that flow FROM this stage to the next. */
		outboundRelationships: string[];
	}

	const stages: PipelineStage[] = [
		{
			key: "observation",
			label: "Observation",
			artifactType: "lesson",
			icon: EyeIcon,
			outboundRelationships: ["observes", "observed-by"],
		},
		{
			key: "understanding",
			label: "Understanding",
			artifactType: "research",
			icon: BookOpenIcon,
			outboundRelationships: ["grounded", "grounded-by"],
		},
		{
			key: "principle",
			label: "Principle",
			artifactType: "decision",
			icon: ScaleIcon,
			outboundRelationships: ["practices", "practiced-by"],
		},
		{
			key: "practice",
			label: "Practice",
			artifactType: "skill",
			icon: WrenchIcon,
			outboundRelationships: ["enforces", "enforced-by"],
		},
		{
			key: "enforcement",
			label: "Enforcement",
			artifactType: "rule",
			icon: ShieldIcon,
			outboundRelationships: ["verifies", "verified-by"],
		},
		{
			key: "verification",
			label: "Verification",
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
	}

	interface EdgeData {
		fromKey: string;
		toKey: string;
		count: number;
	}

	/**
	 * Check whether a reference connects to an artifact in the next stage's type.
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
			if (!isLastStage && count > 0) {
				if (flowRate === 0) {
					status = "stuck";
				} else if (flowRate < 0.3) {
					status = "bottleneck";
				}
			}

			return { stage, artifacts, count, connectedCount, flowRate, status };
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
	<Card.Root class="mb-4">
		<Card.Header class="pb-3">
			<Card.Title class="text-base">
				<div class="flex items-center gap-2">
					<WorkflowIcon class="h-4 w-4 text-muted-foreground" />
					Knowledge Pipeline
				</div>
			</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="flex items-center gap-1 overflow-x-auto pb-2">
				{#each stageDataList as data, i (data.stage.key)}
					<!-- Stage box -->
					<div
						class="flex min-w-[100px] flex-col items-center gap-1.5 rounded-lg border px-3 py-3 {statusBorderClass(data.status)} {statusBgClass(data.status)}"
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

					<!-- Connecting arrow between stages -->
					{#if i < stageDataList.length - 1}
						<div class="flex flex-col items-center gap-0.5 px-1">
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
