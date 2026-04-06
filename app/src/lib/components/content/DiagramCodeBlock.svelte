<script lang="ts">
	import CodeBlock from "$lib/components/content/CodeBlock.svelte";
	import MermaidDiagram from "$lib/components/content/MermaidDiagram.svelte";
	import PlantUmlDiagram from "$lib/components/content/PlantUmlDiagram.svelte";
	import DynamicArtifactTable from "$lib/components/content/DynamicArtifactTable.svelte";
	import { Callout, Caption } from "@orqastudio/svelte-components/pure";

	let {
		text,
		lang = "",
	}: {
		text: string;
		lang?: string;
	} = $props();

	const normalizedLang = $derived(lang?.toLowerCase().trim() ?? "");
	const isMermaid = $derived(normalizedLang === "mermaid");
	const isPlantUml = $derived(normalizedLang === "plantuml" || normalizedLang === "puml");
	const isArtifactTable = $derived(normalizedLang === "artifacts-table");

	/**
	 * Parse key="value" pairs from the artifacts-table text content.
	 * Expected format: type="task" parent="EPIC-067" field="epic"
	 */
	const artifactTableProps = $derived.by(() => {
		if (!isArtifactTable) return { type: "", parent: "", field: "" };
		const attrs: Record<string, string> = {};
		const regex = /(\w+)="([^"]*)"/g;
		let match: RegExpExecArray | null;
		while ((match = regex.exec(text)) !== null) {
			attrs[match[1]] = match[2];
		}
		return {
			type: attrs["type"] ?? "",
			parent: attrs["parent"] ?? "",
			field: attrs["field"] ?? "",
		};
	});
</script>

{#if isMermaid}
	<MermaidDiagram {text} />
{:else if isPlantUml}
	<PlantUmlDiagram {text} />
{:else if isArtifactTable}
	{#if artifactTableProps.type && artifactTableProps.parent && artifactTableProps.field}
		<DynamicArtifactTable
			parentId={artifactTableProps.parent}
			childType={artifactTableProps.type}
			refField={artifactTableProps.field}
		/>
	{:else}
		<Callout tone="warning" border="dashed" iconName="alert-triangle">
			<Caption tone="warning">Invalid artifacts directive: requires type, parent, and field attributes.</Caption>
		</Callout>
	{/if}
{:else}
	<CodeBlock {text} {lang} />
{/if}
