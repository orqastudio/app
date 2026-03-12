<script lang="ts">
	import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
	import Link2OffIcon from "@lucide/svelte/icons/link-2-off";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";

	let { id, path }: { id?: string; path?: string } = $props();

	/** Resolve the display label and whether this link is navigable. */
	const resolved = $derived.by(() => {
		if (id) {
			const node = artifactGraphSDK.resolve(id);
			return { label: id, resolvable: node !== undefined, targetId: node ? id : null };
		}
		if (path) {
			const targetId = artifactGraphSDK.pathIndex.get(path.trim());
			return { label: path, resolvable: targetId !== undefined, targetId: targetId ?? null };
		}
		return { label: "??", resolvable: false, targetId: null };
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
					{resolved.label}
					<ExternalLinkIcon class="h-3 w-3 shrink-0 text-cyan-500/60" />
				</button>
			{/snippet}
		</Tooltip.Trigger>
		<Tooltip.Content side="top">
			<p>Navigate to {resolved.label}</p>
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
