<!-- Renders an agent artifact: description, capabilities, knowledge, model, and markdown body. -->
<script lang="ts">
	import { Icon, SmallBadge, HStack, Stack, Text, Panel } from "@orqastudio/svelte-components/pure";
	import { MetadataRow } from "@orqastudio/svelte-components/pure";
	import { MarkdownRenderer } from "@orqastudio/svelte-components/connected";
	import DiagramCodeBlock from "$lib/components/content/DiagramCodeBlock.svelte";
	import MarkdownLink from "$lib/components/content/MarkdownLink.svelte";
	import { parseFrontmatter } from "$lib/utils/frontmatter";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK } = getStores();
	import { getCapabilityLabel } from "$lib/utils/tool-display";

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
	const capabilities = $derived(
		Array.isArray(metadata.capabilities)
			? (metadata.capabilities as string[]).map(getCapabilityLabel)
			: Array.isArray(metadata.tools)
				? (metadata.tools as string[]).map(getCapabilityLabel)
				: [],
	);
	const knowledge = $derived(
		Array.isArray(metadata.knowledge) ? (metadata.knowledge as string[]) : [],
	);
	const model = $derived(
		typeof metadata.model === "string" ? metadata.model : null,
	);
</script>

<Stack gap={4}>
	<!-- Structured header -->
	<Panel padding="normal" border="bottom"><Stack gap={3}>
		{#if description}
			<Text variant="body-muted" block>{description}</Text>
		{/if}

		<MetadataRow icon="wrench" label="Capabilities" items={capabilities} badgeVariant="secondary" />
		<MetadataRow icon="brain" label="Knowledge" items={knowledge} badgeVariant="outline" />
		{#if model}
			<HStack gap={1}>
				<HStack gap={1}>
					<Icon name="cpu" size="sm" />
					<Text variant="caption">Model</Text>
				</HStack>
				<SmallBadge variant="default">{model}</SmallBadge>
			</HStack>
		{/if}
	</Stack></Panel>

	<!-- Body content -->
	{#if body.trim()}
		<MarkdownRenderer content={body} codeRenderer={DiagramCodeBlock} linkRenderer={MarkdownLink} />
	{/if}
</Stack>
