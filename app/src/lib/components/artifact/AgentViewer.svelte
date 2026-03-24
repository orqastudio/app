<script lang="ts">
	import { Icon, SmallBadge } from "@orqastudio/svelte-components/pure";
	import { MetadataRow } from "@orqastudio/svelte-components/pure";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
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

<div class="space-y-4">
	<!-- Structured header -->
	<div class="space-y-3 border-b border-border pb-4">
		{#if description}
			<p class="text-sm text-muted-foreground">{description}</p>
		{/if}

		<MetadataRow icon="wrench" label="Capabilities" items={capabilities} badgeVariant="secondary" />
		<MetadataRow icon="brain" label="Knowledge" items={knowledge} badgeVariant="outline" />
		{#if model}
			<div class="flex items-center gap-1.5">
				<span class="flex items-center gap-1 text-xs text-muted-foreground">
					<Icon name="cpu" size="sm" />
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
