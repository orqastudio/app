<script lang="ts">
	import SmallBadge from "$lib/components/shared/SmallBadge.svelte";
	import MetadataRow from "$lib/components/shared/MetadataRow.svelte";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import { parseFrontmatter } from "$lib/utils/frontmatter";
	import WrenchIcon from "@lucide/svelte/icons/wrench";
	import SparklesIcon from "@lucide/svelte/icons/sparkles";
	import CpuIcon from "@lucide/svelte/icons/cpu";

	let { content }: { content: string } = $props();

	const parsed = $derived(parseFrontmatter(content));
	const description = $derived(
		typeof parsed.metadata.description === "string"
			? parsed.metadata.description
			: null,
	);
	const tools = $derived(
		Array.isArray(parsed.metadata.tools) ? parsed.metadata.tools : [],
	);
	const skills = $derived(
		Array.isArray(parsed.metadata.skills) ? parsed.metadata.skills : [],
	);
	const model = $derived(
		typeof parsed.metadata.model === "string" ? parsed.metadata.model : null,
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
	{#if parsed.body.trim()}
		<MarkdownRenderer content={parsed.body} />
	{/if}
</div>
