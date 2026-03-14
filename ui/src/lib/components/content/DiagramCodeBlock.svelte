<script lang="ts">
	import CodeBlock from "$lib/components/content/CodeBlock.svelte";
	import MermaidDiagram from "$lib/components/content/MermaidDiagram.svelte";
	import PlantUmlDiagram from "$lib/components/content/PlantUmlDiagram.svelte";

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
</script>

{#if isMermaid}
	<MermaidDiagram {text} />
{:else if isPlantUml}
	<PlantUmlDiagram {text} />
{:else}
	<CodeBlock {text} {lang} />
{/if}
