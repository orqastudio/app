<script lang="ts">
	import { SmallBadge, HStack } from "@orqastudio/svelte-components/pure";
	import { MetadataRow } from "@orqastudio/svelte-components/pure";
	import { MarkdownRenderer } from "@orqastudio/svelte-components/connected";
	import DiagramCodeBlock from "$lib/components/content/DiagramCodeBlock.svelte";
	import MarkdownLink from "$lib/components/content/MarkdownLink.svelte";
	import { parseFrontmatter } from "$lib/utils/frontmatter";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK } = getStores();

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
	const allowedTools = $derived(
		Array.isArray(metadata["allowed-tools"])
			? (metadata["allowed-tools"] as string[])
			: [],
	);
	const version = $derived(
		typeof metadata.version === "string" ? metadata.version : null,
	);
	const tags = $derived(
		Array.isArray(metadata.tags) ? (metadata.tags as string[]) : [],
	);
</script>

<div class="space-y-4">
	<!-- Structured header -->
	<div class="space-y-3 border-b border-border pb-4">
		{#if version}
			<HStack gap={1}>
				<SmallBadge variant="outline">v{version}</SmallBadge>
			</HStack>
		{/if}

		{#if description}
			<p class="text-sm text-muted-foreground">{description}</p>
		{/if}

		<MetadataRow icon="wrench" label="Allowed Tools" items={allowedTools} badgeVariant="secondary" />
		<MetadataRow icon="tag" label="Tags" items={tags} badgeVariant="outline" />
	</div>

	<!-- Body content -->
	{#if body.trim()}
		<MarkdownRenderer content={body} codeRenderer={DiagramCodeBlock} linkRenderer={MarkdownLink} />
	{/if}
</div>
