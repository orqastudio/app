<script lang="ts">
	import SmallBadge from "$lib/components/shared/SmallBadge.svelte";
	import MetadataRow from "$lib/components/shared/MetadataRow.svelte";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import { parseFrontmatter } from "$lib/utils/frontmatter";
	import WrenchIcon from "@lucide/svelte/icons/wrench";
	import TagIcon from "@lucide/svelte/icons/tag";

	let { content }: { content: string } = $props();

	const parsed = $derived(parseFrontmatter(content));
	const description = $derived(
		typeof parsed.metadata.description === "string"
			? parsed.metadata.description
			: null,
	);
	const allowedTools = $derived(
		Array.isArray(parsed.metadata["allowed-tools"])
			? parsed.metadata["allowed-tools"]
			: [],
	);
	const version = $derived(
		typeof parsed.metadata.version === "string"
			? parsed.metadata.version
			: null,
	);
	const tags = $derived(
		Array.isArray(parsed.metadata.tags) ? parsed.metadata.tags : [],
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
	{#if parsed.body.trim()}
		<MarkdownRenderer content={parsed.body} />
	{/if}
</div>
