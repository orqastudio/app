<script lang="ts">
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK, navigationStore, projectStore } = getStores();
	import {
		statusIconName,
		resolveIcon,
		Table,
		TableHeader,
		TableBody,
		TableRow,
		TableHead,
		TableCell,
		Badge,
		Caption,
		Panel,
	} from "@orqastudio/svelte-components/pure";
	import type { ArtifactNode } from "@orqastudio/types";

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

	/**
	 * Priority sort order — P1 first, P2, P3, unset last.
	 * Derived inline as a fixed platform constant (P1/P2/P3 are platform-level).
	 */
	const PRIORITY_ORDER: Record<string, number> = { P1: 0, P2: 1, P3: 2 };

	/**
	 * Status sort order derived from project settings.
	 * Array index is the natural sort position — no static config needed.
	 */
	const statusOrder = $derived(
		Object.fromEntries((projectStore.projectSettings?.statuses ?? []).map((s, i) => [s.key, i])),
	);

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
			const sa = statusOrder[a.status ?? ""] ?? 50;
			const sb = statusOrder[b.status ?? ""] ?? 50;
			return sa - sb;
		});

		return matched;
	});

	/**
	 * Navigate to the artifact with the given ID in the artifact viewer.
	 * @param id - The artifact ID to open in the viewer.
	 */
	function navigateTo(id: string): void {
		navigationStore.navigateToArtifact(id);
	}
</script>

{#if children.length > 0}
	<Panel padding="none" border="all" rounded="lg">
		<Table>
			<TableHeader>
				<TableRow>
					<TableHead width="xs"></TableHead>
					<TableHead>ID</TableHead>
					<TableHead>Title</TableHead>
					<TableHead>Priority</TableHead>
					<TableHead>Status</TableHead>
				</TableRow>
			</TableHeader>
			<TableBody>
				{#each children as child (child.id)}
					<TableRow
						interactive
						onclick={() => navigateTo(child.id)}
						role="button"
						tabindex={0}
						onkeydown={(e) => {
							if (e.key === "Enter" || e.key === " ") navigateTo(child.id);
						}}
					>
						<!-- Status icon -->
						<TableCell>
							{#if child.status}
								{@const StatusIcon = resolveIcon(statusIconName(child.status))}
								<StatusIcon class="text-muted-foreground h-3.5 w-3.5" />
							{/if}
						</TableCell>
						<!-- ID -->
						<TableCell mono>
							{child.id}
						</TableCell>
						<!-- Title -->
						<TableCell>
							{child.title}
						</TableCell>
						<!-- Priority -->
						<TableCell>
							{#if child.priority}
								<Badge variant="secondary">{child.priority}</Badge>
							{:else}
								<Caption tone="muted">--</Caption>
							{/if}
						</TableCell>
						<!-- Status -->
						<TableCell>
							{child.status ?? "--"}
						</TableCell>
					</TableRow>
				{/each}
			</TableBody>
		</Table>
	</Panel>
{:else}
	<!-- Inline style required: dashed border-style and text-align cannot be expressed via Box typed props -->
	<div
		style="margin-top: 1rem; border-radius: 0.5rem; border: 1px dashed hsl(var(--border)); padding: 1rem; text-align: center;"
	>
		<Caption tone="muted">No {childType} artifacts found for {parentId}</Caption>
	</div>
{/if}
