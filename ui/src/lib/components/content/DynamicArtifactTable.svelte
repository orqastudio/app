<script lang="ts">
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { statusIcon } from "$lib/components/shared/StatusIndicator.svelte";
	import type { ArtifactNode } from "$lib/types/artifact-graph";

	let {
		parentId,
		childType,
		refField,
	}: {
		/** The artifact ID whose children to show (e.g. "EPIC-067"). */
		parentId: string;
		/** The artifact type of children to find (e.g. "task"). */
		childType: string;
		/** The frontmatter field that links children to parent (e.g. "epic"). */
		refField: string;
	} = $props();

	/** Priority sort order (P1 first, then P2, P3, unset last). */
	const PRIORITY_ORDER: Record<string, number> = { P1: 0, P2: 1, P3: 2 };

	/** Status sort order for secondary sorting. */
	const STATUS_ORDER: Record<string, number> = {
		active: 0,
		review: 1,
		ready: 2,
		prioritised: 3,
		exploring: 4,
		captured: 5,
		blocked: 6,
		hold: 7,
		completed: 8,
		surpassed: 9,
	};

	/** Find all artifacts of childType where frontmatter[refField] matches parentId. */
	const children = $derived.by((): ArtifactNode[] => {
		const candidates = artifactGraphSDK.byType(childType);
		const matched = candidates.filter((node) => {
			const fieldValue = node.frontmatter[refField];
			if (typeof fieldValue === "string") {
				return fieldValue === parentId;
			}
			if (Array.isArray(fieldValue)) {
				return fieldValue.includes(parentId);
			}
			return false;
		});

		// Sort by priority (P1 first), then by status
		matched.sort((a, b) => {
			const pa = PRIORITY_ORDER[a.priority ?? ""] ?? 99;
			const pb = PRIORITY_ORDER[b.priority ?? ""] ?? 99;
			if (pa !== pb) return pa - pb;
			const sa = STATUS_ORDER[a.status ?? ""] ?? 50;
			const sb = STATUS_ORDER[b.status ?? ""] ?? 50;
			return sa - sb;
		});

		return matched;
	});

	function navigateTo(id: string): void {
		navigationStore.navigateToArtifact(id);
	}
</script>

{#if children.length > 0}
	<div class="my-4 overflow-hidden rounded-lg border">
		<table class="w-full text-sm">
			<thead>
				<tr class="border-b bg-muted/50 text-left text-xs text-muted-foreground">
					<th class="w-6 px-3 py-2"></th>
					<th class="px-3 py-2">ID</th>
					<th class="px-3 py-2">Title</th>
					<th class="px-3 py-2">Priority</th>
					<th class="px-3 py-2">Status</th>
				</tr>
			</thead>
			<tbody>
				{#each children as child (child.id)}
					<tr
						class="cursor-pointer border-b last:border-b-0 hover:bg-muted/30 transition-colors"
						onclick={() => navigateTo(child.id)}
						role="button"
						tabindex="0"
						onkeydown={(e) => { if (e.key === "Enter" || e.key === " ") navigateTo(child.id); }}
					>
						<!-- Status dot -->
						<td class="px-3 py-2">
							{#if child.status}
								{@const StatusIcon = statusIcon(child.status)}
								<StatusIcon class="h-3.5 w-3.5 text-muted-foreground" />
							{/if}
						</td>
						<!-- ID -->
						<td class="px-3 py-2 font-mono text-xs text-muted-foreground">
							{child.id}
						</td>
						<!-- Title -->
						<td class="px-3 py-2 font-medium">
							{child.title}
						</td>
						<!-- Priority -->
						<td class="px-3 py-2">
							{#if child.priority}
								<span class="rounded bg-muted px-1.5 py-0.5 text-xs font-medium">
									{child.priority}
								</span>
							{:else}
								<span class="text-xs text-muted-foreground">--</span>
							{/if}
						</td>
						<!-- Status -->
						<td class="px-3 py-2 text-xs capitalize text-muted-foreground">
							{child.status ?? "--"}
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{:else}
	<div class="my-4 rounded-lg border border-dashed p-4 text-center text-sm text-muted-foreground">
		No {childType} artifacts found for {parentId}
	</div>
{/if}
