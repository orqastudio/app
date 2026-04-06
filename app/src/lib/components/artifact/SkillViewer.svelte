<!-- Renders a skill artifact: version badge, description, allowed tools, tags, and markdown body. -->
<script lang="ts">
	import { SmallBadge, HStack, Stack, Text, Panel } from "@orqastudio/svelte-components/pure";
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

<Stack gap={4}>
	<!-- Structured header -->
	<Panel padding="normal" border="bottom"><Stack gap={3}>
		{#if version}
			<HStack gap={1}>
				<SmallBadge variant="outline">v{version}</SmallBadge>
			</HStack>
		{/if}

		{#if description}
			<Text variant="body-muted" block>{description}</Text>
		{/if}

		<MetadataRow icon="wrench" label="Allowed Tools" items={allowedTools} badgeVariant="secondary" />
		<MetadataRow icon="tag" label="Tags" items={tags} badgeVariant="outline" />
	</Stack></Panel>

	<!-- Body content -->
	{#if body.trim()}
		<MarkdownRenderer content={body} codeRenderer={DiagramCodeBlock} linkRenderer={MarkdownLink} />
	{/if}
</Stack>
