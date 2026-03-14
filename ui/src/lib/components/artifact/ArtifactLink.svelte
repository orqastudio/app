<script lang="ts">
	import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
	import Link2OffIcon from "@lucide/svelte/icons/link-2-off";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { statusColor } from "$lib/components/shared/StatusIndicator.svelte";

	let { id, path, displayLabel }: { id?: string; path?: string; displayLabel?: string } = $props();

	/** Resolve the display label, node metadata, and whether this link is navigable. */
	const resolved = $derived.by(() => {
		if (id) {
			const node = artifactGraphSDK.resolve(id);
			const label = displayLabel ?? id;
			return { label, resolvable: node !== undefined, targetId: node ? id : null, node: node ?? null };
		}
		if (path) {
			const targetId = artifactGraphSDK.pathIndex.get(path.trim());
			const node = targetId ? artifactGraphSDK.resolve(targetId) : undefined;
			const label = displayLabel ?? path;
			return { label, resolvable: targetId !== undefined, targetId: targetId ?? null, node: node ?? null };
		}
		return { label: displayLabel ?? "??", resolvable: false, targetId: null, node: null };
	});

	/** Whether the label differs from the raw ID (i.e. we're showing a title). */
	const showingTitle = $derived(
		resolved.node !== null && resolved.label !== resolved.targetId,
	);

	/** Status dot colour class for the resolved node, or null if no status. */
	const dotClass = $derived(
		resolved.node?.status ? statusColor(resolved.node.status) : null,
	);

	/** First line of the description for use in the popover. */
	const descriptionSnippet = $derived.by(() => {
		const desc = resolved.node?.description;
		if (!desc) return null;
		const firstLine = desc.split("\n")[0].trim();
		return firstLine.length > 120 ? firstLine.slice(0, 120) + "…" : firstLine;
	});

	function handleClick() {
		if (resolved.targetId) {
			navigationStore.navigateToArtifact(resolved.targetId);
		}
	}
</script>

{#if resolved.resolvable}
	<Tooltip.Root>
		<Tooltip.Trigger>
			{#snippet child({ props })}
				<button
					{...props}
					class="inline-flex items-center gap-1 whitespace-nowrap rounded border border-cyan-500/30 bg-cyan-500/10 px-1.5 py-0.5 font-mono text-[11px] font-medium text-cyan-400 transition-all hover:border-cyan-400 hover:bg-cyan-500/20"
					onclick={handleClick}
				>
					{#if dotClass}
						<span class="inline-block h-1.5 w-1.5 shrink-0 rounded-full {dotClass}"></span>
					{/if}
					{#if showingTitle}
						<span class="max-w-[200px] overflow-hidden text-ellipsis whitespace-nowrap">{resolved.label}</span>
					{:else}
						{resolved.label}
					{/if}
					<ExternalLinkIcon class="h-3 w-3 shrink-0 text-cyan-500/60" />
				</button>
			{/snippet}
		</Tooltip.Trigger>
		<Tooltip.Content side="top" class="max-w-xs">
			{#if resolved.node}
				{@const node = resolved.node}
				<div class="space-y-1 text-xs">
					<div class="flex items-center gap-1.5">
						{#if dotClass}
							<span class="inline-block h-1.5 w-1.5 shrink-0 rounded-full {dotClass}"></span>
						{/if}
						<span class="font-mono font-semibold">{node.id}</span>
						{#if node.status}
							<span class="capitalize text-muted-foreground">· {node.status}</span>
						{/if}
					</div>
					{#if node.title && node.title !== node.id}
						<p class="font-medium leading-snug">{node.title}</p>
					{/if}
					{#if node.artifact_type}
						<p class="capitalize text-muted-foreground">{node.artifact_type}</p>
					{/if}
					{#if descriptionSnippet}
						<p class="text-muted-foreground">{descriptionSnippet}</p>
					{/if}
				</div>
			{:else}
				<p>Navigate to {resolved.label}</p>
			{/if}
		</Tooltip.Content>
	</Tooltip.Root>
{:else}
	<Tooltip.Root>
		<Tooltip.Trigger>
			{#snippet child({ props })}
				<span
					{...props}
					class="inline-flex items-center gap-1 whitespace-nowrap rounded border border-warning/30 bg-warning/10 px-1.5 py-0.5 font-mono text-[11px] font-medium text-warning"
				>
					<Link2OffIcon class="h-3 w-3 shrink-0 text-muted-foreground" />
					{resolved.label}
				</span>
			{/snippet}
		</Tooltip.Trigger>
		<Tooltip.Content side="top">
			<p>Not found in artifact graph: {resolved.label}</p>
		</Tooltip.Content>
	</Tooltip.Root>
{/if}
