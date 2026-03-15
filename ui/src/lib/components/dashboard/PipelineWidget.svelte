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
	import PipelineStages, { type PipelineStage, type PipelineEdge } from "$lib/components/shared/PipelineStages.svelte";

	// -------------------------------------------------------------------------
	// Pipeline stage definitions
	// -------------------------------------------------------------------------

	interface StageDef {
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

	const stageDefs: StageDef[] = [
		{
			key: "observation",
			label: "Observation",
			artifactNoun: "lessons",
			artifactType: "lesson",
			icon: EyeIcon,
			outboundRelationships: ["informs", "informed-by", "grounded", "grounded-by"],
		},
		{
			key: "understanding",
			label: "Understanding",
			artifactNoun: "research docs",
			artifactType: "research",
			icon: BookOpenIcon,
			outboundRelationships: ["grounded", "grounded-by", "informs", "informed-by"],
		},
		{
			key: "principle",
			label: "Principle",
			artifactNoun: "decisions",
			artifactType: "decision",
			icon: ScaleIcon,
			outboundRelationships: ["practices", "practiced-by"],
		},
		{
			key: "practice",
			label: "Practice",
			artifactNoun: "skills",
			artifactType: "skill",
			icon: WrenchIcon,
			outboundRelationships: ["enforces", "enforced-by"],
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
		def: StageDef;
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

	interface EdgeCount {
		fromKey: string;
		toKey: string;
		count: number;
	}

	function refConnectsToType(ref: ArtifactRef, targetType: string | null): boolean {
		if (targetType === null) return false;
		const targetNode = artifactGraphSDK.resolve(ref.target_id);
		return targetNode?.artifact_type === targetType;
	}

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

	function hasAnyRelationship(artifact: ArtifactNode): boolean {
		return artifact.references_out.length > 0 || artifact.references_in.length > 0;
	}

	const stageDataList = $derived.by((): StageData[] => {
		return stageDefs.map((def) => {
			const artifacts =
				def.artifactType !== null ? artifactGraphSDK.byType(def.artifactType) : [];
			const count = artifacts.length;

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
					reason = `${orphanCount} of ${count} ${def.artifactNoun} have no relationships`;
					suggestion = `Review orphaned ${def.artifactNoun} and add cross-references`;
				} else if (connectivity < 0.7) {
					status = "attention";
					reason = `${orphanCount} of ${count} ${def.artifactNoun} have no relationships`;
					suggestion = `Connect isolated ${def.artifactNoun} to related artifacts`;
				}
			}

			return { def, artifacts, count, connectedCount, connectivity, status, reason, suggestion };
		});
	});

	const edgeCountList = $derived.by((): EdgeCount[] => {
		const edges: EdgeCount[] = [];
		for (let i = 0; i < stageDefs.length - 1; i++) {
			const fromDef = stageDefs[i];
			const toDef = stageDefs[i + 1];
			const fromArtifacts =
				fromDef.artifactType !== null
					? artifactGraphSDK.byType(fromDef.artifactType)
					: [];
			const count = countEdgesBetween(
				fromArtifacts,
				toDef.artifactType,
				fromDef.outboundRelationships
			);
			edges.push({ fromKey: fromDef.key, toKey: toDef.key, count });
		}
		return edges;
	});

	const hasData = $derived(artifactGraphSDK.graph.size > 0);

	// -------------------------------------------------------------------------
	// Visual helpers → PipelineStage prop mapping
	// -------------------------------------------------------------------------

	function statusBorderClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":  return "border-red-400 dark:border-red-600";
			case "attention": return "border-amber-400 dark:border-amber-600";
			default:          return "border-border";
		}
	}

	function statusBgClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":  return "bg-red-50 dark:bg-red-950/30";
			case "attention": return "bg-amber-50 dark:bg-amber-950/30";
			default:          return "bg-muted/30";
		}
	}

	function computeStatusLabel(data: StageData): string | null {
		if (data.count === 0) return null;
		const pct = Math.round(data.connectivity * 100);
		if (data.status === "isolated" || data.status === "attention") {
			return `${pct}% connected`;
		}
		return null;
	}

	function statusLabelClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":  return "text-red-500";
			case "attention": return "text-amber-500";
			default:          return "text-muted-foreground";
		}
	}

	// Map status to dot color class — matches LessonVelocityWidget's visual pattern
	function statusDotColorClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":  return "bg-red-500";
			case "attention": return "bg-amber-500";
			default:          return "bg-muted-foreground/50";
		}
	}

	const pipelineStages = $derived.by((): PipelineStage[] =>
		stageDataList.map((data) => ({
			key: data.def.key,
			label: data.def.label,
			count: data.count,
			// Use dotColorClass (coloured circle) to match LessonVelocityWidget pattern
			dotColorClass: statusDotColorClass(data.status),
			borderClass: statusBorderClass(data.status),
			bgClass: statusBgClass(data.status),
			statusLabel: computeStatusLabel(data),
			statusLabelClass: statusLabelClass(data.status),
			tooltipTitle: data.reason,
			tooltipBody: data.suggestion,
		}))
	);

	const pipelineEdges = $derived.by((): PipelineEdge[] =>
		edgeCountList.map((e) => ({ count: e.count }))
	);
</script>

{#if hasData}
	<Card.Root class="gap-2 h-full">
		<Card.Header class="pb-2">
			<Card.Title class="text-sm font-semibold">
				<div class="flex items-center gap-2">
					<WorkflowIcon class="h-4 w-4 text-muted-foreground" />
					Knowledge Pipeline
				</div>
			</Card.Title>
		</Card.Header>
		<Card.Content class="pt-0">
			<div class="pb-2">
				<PipelineStages stages={pipelineStages} edges={pipelineEdges} />
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
