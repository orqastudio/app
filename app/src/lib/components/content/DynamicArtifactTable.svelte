<script lang="ts">
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK, navigationStore } = getStores();
	import { statusIconName, resolveIcon } from "@orqastudio/svelte-components/pure";
	import type { ArtifactNode } from "@orqastudio/types";
	import { PRIORITY_ORDER, STATUS_ORDER } from "$lib/config/sort-orders";

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
								{@const StatusIcon = resolveIcon(statusIconName(child.status))}
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
