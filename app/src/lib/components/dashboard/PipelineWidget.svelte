<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardContent } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK, pluginRegistry } = getStores();
	import type { ArtifactNode, ArtifactRef, RelationshipType } from "@orqastudio/types";

	import { PipelineStages, type PipelineStage, type PipelineEdge } from "@orqastudio/svelte-components/pure";

	// -------------------------------------------------------------------------
	// Pipeline stage definitions — derived from registered relationships
	// -------------------------------------------------------------------------

	interface StageDef {
		key: string;
		label: string;
		artifactNoun: string;
		artifactType: string;
		icon: string;
		/** Relationship keys that flow FROM this stage's artifacts to the next stage. */
		outboundRelationships: string[];
	}

	/**
	 * Build pipeline stages from the registry's governance semantic.
	 *
	 * The governance learning loop is:
	 *   lesson →(teaches)→ decision →(governs)→ rule →(enforces)→ decision
	 *   rule →(codifies)→ lesson (closing the loop)
	 *
	 * We also include knowledge-flow into decisions:
	 *   research →(informs)→ decision
	 *
	 * Stages are built by finding which artifact types appear as `from`
	 * in governance + knowledge-flow relationships.
	 */
	const stageDefs = $derived.by((): StageDef[] => {
		const allRels = pluginRegistry.allRelationships;

		// Find relationships by semantic that form the governance pipeline
		const findRels = (semantic: string): RelationshipType[] =>
			allRels.filter((r) => r.semantic === semantic);

		const govRels = findRels("governance");
		const knowledgeRels = findRels("knowledge-flow");

		// Build stages from the types that participate in governance + knowledge flow
		const stages: StageDef[] = [];

		// Lesson stage — lessons teach decisions, lessons get codified into rules
		const lessonOutbound = [
			...knowledgeRels.filter((r) => r.from.includes("lesson")).map((r) => r.key),
			...knowledgeRels.filter((r) => r.from.includes("lesson")).map((r) => r.inverse),
		];
		if (lessonOutbound.length > 0) {
			stages.push({
				key: "lesson",
				label: "Learning",
				artifactNoun: "lessons",
				artifactType: "lesson",
				icon: "book-open",
				outboundRelationships: lessonOutbound,
			});
		}

		// Research stage — research informs decisions and guides epics
		const researchOutbound = [
			...knowledgeRels.filter((r) => r.from.includes("research")).map((r) => r.key),
			...knowledgeRels.filter((r) => r.from.includes("research")).map((r) => r.inverse),
		];
		if (researchOutbound.length > 0) {
			stages.push({
				key: "research",
				label: "Research",
				artifactNoun: "research docs",
				artifactType: "research",
				icon: "flask-conical",
				outboundRelationships: researchOutbound,
			});
		}

		// Decision stage — decisions drive epics and govern rules
		const decisionOutbound = [
			...govRels.filter((r) => r.from.includes("decision")).map((r) => r.key),
			...govRels.filter((r) => r.from.includes("decision")).map((r) => r.inverse),
		];
		if (decisionOutbound.length > 0) {
			stages.push({
				key: "decision",
				label: "Decisions",
				artifactNoun: "decisions",
				artifactType: "decision",
				icon: "scale",
				outboundRelationships: decisionOutbound,
			});
		}

		// Rule stage — rules enforce decisions, codify lessons
		const ruleOutbound = [
			...govRels.filter((r) => r.from.includes("rule")).map((r) => r.key),
			...govRels.filter((r) => r.from.includes("rule")).map((r) => r.inverse),
		];
		if (ruleOutbound.length > 0) {
			stages.push({
				key: "rule",
				label: "Rules",
				artifactNoun: "rules",
				artifactType: "rule",
				icon: "shield",
				outboundRelationships: ruleOutbound,
			});
		}

		return stages;
	});

	// -------------------------------------------------------------------------
	// Computed pipeline data — connectivity model
	// -------------------------------------------------------------------------

	interface StageData {
		def: StageDef;
		artifacts: ArtifactNode[];
		count: number;
		connectedCount: number;
		connectivity: number;
		status: "healthy" | "attention" | "isolated";
		reason: string | null;
		suggestion: string | null;
	}

	interface EdgeCount {
		fromKey: string;
		toKey: string;
		count: number;
	}

	function refConnectsToType(ref: ArtifactRef, targetType: string): boolean {
		const targetNode = artifactGraphSDK.resolve(ref.target_id);
		return targetNode?.artifact_type === targetType;
	}

	function countEdgesBetween(
		fromArtifacts: ArtifactNode[],
		toType: string,
		relationshipTypes: string[]
	): number {
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
			const artifacts = artifactGraphSDK.byType(def.artifactType);
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
			const fromArtifacts = artifactGraphSDK.byType(fromDef.artifactType);
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
	// Visual helpers
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
	<CardRoot class="gap-2 h-full">
		<CardHeader class="pb-2">
			<CardTitle class="text-sm font-semibold">
				<div class="flex items-center gap-2">
					<Icon name="workflow" size="md" />
					Governance Pipeline
				</div>
			</CardTitle>
		</CardHeader>
		<CardContent class="pt-0">
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
		</CardContent>
	</CardRoot>
{/if}
