<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardContent } from "@orqastudio/svelte-components/pure";
	import { getStores, pct } from "@orqastudio/sdk";

	const { artifactGraphSDK, pluginRegistry } = getStores();
	import type { ArtifactNode, ArtifactRef, RelationshipType } from "@orqastudio/types";
	import { assertNever } from "@orqastudio/types";

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

		// Build stages from the types that participate in governance + knowledge flow.
		// Artifact type keys are read from the plugin registry so no static config import is needed.
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
				icon: pluginRegistry.getIconForType("lesson"),
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
				icon: pluginRegistry.getIconForType("research"),
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
				icon: pluginRegistry.getIconForType("decision"),
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
				icon: pluginRegistry.getIconForType("rule"),
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

	/**
	 * Returns true if the ref's target artifact is of the given type.
	 * @param ref - The artifact reference to check.
	 * @param targetType - The artifact type key to match against.
	 * @returns Whether the referenced artifact is of the target type.
	 */
	function refConnectsToType(ref: ArtifactRef, targetType: string): boolean {
		const targetNode = artifactGraphSDK.resolve(ref.target_id);
		return targetNode?.artifact_type === targetType;
	}

	/**
	 * Counts outbound edges between a set of artifacts and a given target type.
	 * Only counts edges whose relationship type is in the provided list.
	 * @param fromArtifacts - The source artifacts to check edges from.
	 * @param toType - The artifact type of the target to count edges toward.
	 * @param relationshipTypes - The relationship type keys to include.
	 * @returns The total number of matching outbound edges.
	 */
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

	/**
	 * Returns true if the artifact has at least one inbound or outbound reference.
	 * @param artifact - The artifact node to check.
	 * @returns Whether the artifact participates in any relationship.
	 */
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

	/**
	 * Returns the Tailwind border class for a stage status.
	 * @param status - The pipeline stage connectivity status.
	 * @returns The CSS border class string.
	 */
	function statusBorderClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":  return "border-destructive";
			case "attention": return "border-warning";
			case "healthy":   return "border-border";
			default:          return assertNever(status);
		}
	}

	/**
	 * Returns the Tailwind background class for a stage status.
	 * @param status - The pipeline stage connectivity status.
	 * @returns The CSS background class string.
	 */
	function statusBgClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":  return "bg-destructive/10";
			case "attention": return "bg-warning/10";
			case "healthy":   return "bg-muted/30";
			default:          return assertNever(status);
		}
	}

	/**
	 * Computes the connectivity percentage label for a stage, or null if healthy/empty.
	 * @param data - The stage data including count, connectivity, and status.
	 * @returns A formatted label string or null if no label should be shown.
	 */
	function computeStatusLabel(data: StageData): string | null {
		if (data.count === 0) return null;
		if (data.status === "isolated" || data.status === "attention") {
			return `${pct(data.connectivity)}% connected`;
		}
		return null;
	}

	/**
	 * Returns the Tailwind text color class for a stage status label.
	 * @param status - The pipeline stage connectivity status.
	 * @returns The CSS text color class string.
	 */
	function statusLabelClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":  return "text-destructive";
			case "attention": return "text-warning";
			case "healthy":   return "text-muted-foreground";
			default:          return assertNever(status);
		}
	}

	/**
	 * Returns the Tailwind background color class for a stage status dot indicator.
	 * @param status - The pipeline stage connectivity status.
	 * @returns The CSS background color class string.
	 */
	function statusDotColorClass(status: StageData["status"]): string {
		switch (status) {
			case "isolated":  return "bg-destructive";
			case "attention": return "bg-warning";
			case "healthy":   return "bg-muted-foreground/50";
			default:          return assertNever(status);
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
	<CardRoot full>
		<CardHeader compact>
			<CardTitle>
				<div class="flex items-center gap-2">
					<Icon name="workflow" size="md" />
					Governance Pipeline
				</div>
			</CardTitle>
		</CardHeader>
		<CardContent>
			<div class="pb-2">
				<PipelineStages stages={pipelineStages} edges={pipelineEdges} />
			</div>

			<!-- Legend -->
			<div class="mt-3 flex items-center gap-4 text-[10px] text-muted-foreground">
				<div class="flex items-center gap-1">
					<span class="inline-block h-2 w-2 rounded-full bg-destructive"></span>
					Isolated (&lt;30% connected)
				</div>
				<div class="flex items-center gap-1">
					<span class="inline-block h-2 w-2 rounded-full bg-warning"></span>
					Attention (30-70% connected)
				</div>
			</div>
		</CardContent>
	</CardRoot>
{/if}
