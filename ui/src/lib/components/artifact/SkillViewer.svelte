<script lang="ts">
	import SmallBadge from "$lib/components/shared/SmallBadge.svelte";
	import MetadataRow from "$lib/components/shared/MetadataRow.svelte";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import { parseFrontmatter } from "$lib/utils/frontmatter";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import WrenchIcon from "@lucide/svelte/icons/wrench";
	import TagIcon from "@lucide/svelte/icons/tag";

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
			<div class="flex items-center gap-1.5">
				<SmallBadge variant="outline">v{version}</SmallBadge>
			</div>
		{/if}

		{#if description}
			<p class="text-sm text-muted-foreground">{description}</p>
		{/if}

		<MetadataRow icon={WrenchIcon} label="Allowed Tools" items={allowedTools} badgeVariant="secondary" />
		<MetadataRow icon={TagIcon} label="Tags" items={tags} badgeVariant="outline" />
	</div>

	<!-- Body content -->
	{#if body.trim()}
		<MarkdownRenderer content={body} />
	{/if}
</div>
