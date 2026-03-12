<script lang="ts">
	import SmallBadge from "$lib/components/shared/SmallBadge.svelte";
	import MetadataRow from "$lib/components/shared/MetadataRow.svelte";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import { parseFrontmatter } from "$lib/utils/frontmatter";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import WrenchIcon from "@lucide/svelte/icons/wrench";
	import SparklesIcon from "@lucide/svelte/icons/sparkles";
	import CpuIcon from "@lucide/svelte/icons/cpu";

	let { content, path }: { content: string; path?: string } = $props();

	/**
	 * Graph node for this artifact, when available.
	 * Undefined for files not yet indexed by the watcher.
	 */
	const graphNode = $derived(path ? artifactGraphSDK.resolveByPath(path) : undefined);

	/**
	 * Effective metadata: prefer pre-parsed frontmatter from the graph when
	 * available; fall back to parsing the raw content string.
	 */
	const metadata = $derived.by(() => {
		if (graphNode) {
			return graphNode.frontmatter as Record<string, unknown>;
		}
		return parseFrontmatter(content).metadata as Record<string, unknown>;
	});

	/**
	 * Body: always parsed from raw content so the markdown portion is correct.
	 */
	const body = $derived(parseFrontmatter(content).body);

	const description = $derived(
		typeof metadata.description === "string" ? metadata.description : null,
	);
	const tools = $derived(
		Array.isArray(metadata.tools) ? (metadata.tools as string[]) : [],
	);
	const skills = $derived(
		Array.isArray(metadata.skills) ? (metadata.skills as string[]) : [],
	);
	const model = $derived(
		typeof metadata.model === "string" ? metadata.model : null,
	);
</script>

<div class="space-y-4">
	<!-- Structured header -->
	<div class="space-y-3 border-b border-border pb-4">
		{#if description}
			<p class="text-sm text-muted-foreground">{description}</p>
		{/if}

		<MetadataRow icon={WrenchIcon} label="Tools" items={tools} badgeVariant="secondary" />
		<MetadataRow icon={SparklesIcon} label="Skills" items={skills} badgeVariant="outline" />
		{#if model}
			<div class="flex items-center gap-1.5">
				<span class="flex items-center gap-1 text-xs text-muted-foreground">
					<CpuIcon class="h-3.5 w-3.5" />
					Model
				</span>
				<SmallBadge variant="default">{model}</SmallBadge>
			</div>
		{/if}
	</div>

	<!-- Body content -->
	{#if body.trim()}
		<MarkdownRenderer content={body} />
	{/if}
</div>
