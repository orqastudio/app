<script lang="ts">
	import { Icon, TooltipRoot, TooltipTrigger, TooltipContent } from "../../pure/index.js";
	import { getStores } from "@orqastudio/sdk";
	import { statusIconName, statusIsSpinning, resolveIcon } from "../../pure/index.js";


	const { navigationStore, artifactGraphSDK, projectStore } = getStores();

	let { id, path, displayLabel }: { id?: string; path?: string; displayLabel?: string } = $props();

	/** Extract the project prefix from a qualified ID (e.g. "sdk::EPIC-001" -> "sdk"). */
	function extractProjectPrefix(qualifiedId: string): string | null {
		const idx = qualifiedId.indexOf("::");
		return idx >= 0 ? qualifiedId.slice(0, idx) : null;
	}

	/** Resolve the display label, node metadata, and whether this link is navigable. */
	const resolved = $derived.by(() => {
		if (id) {
			const node = artifactGraphSDK.resolve(id);
			const label = displayLabel ?? (id.includes("::") ? id.split("::")[1] : id);
			return { label, resolvable: node !== undefined, targetId: node ? id : null, node: node ?? null, projectPrefix: extractProjectPrefix(id) };
		}
		if (path) {
			const targetId = artifactGraphSDK.pathIndex.get(path.trim());
			const node = targetId ? artifactGraphSDK.resolve(targetId) : undefined;
			const label = displayLabel ?? path;
			return { label, resolvable: targetId !== undefined, targetId: targetId ?? null, node: node ?? null, projectPrefix: null };
		}
		return { label: displayLabel ?? "??", resolvable: false, targetId: null, node: null, projectPrefix: null };
	});

	/** The type prefix of the resolved artifact (e.g. "EPIC" from "EPIC-001"). */
	const typePrefix = $derived.by((): string | null => {
		const targetId = resolved.targetId;
		if (!targetId) return null;
		const match = /^([A-Z]+)-\d+$/.exec(targetId);
		return match ? match[1] : null;
	});

	/** Effective display mode for this artifact type from project settings (default: "id"). */
	const displayMode = $derived.by((): "id" | "title" => {
		const modes = projectStore.projectSettings?.artifactLinks?.displayModes ?? {};
		if (typePrefix && modes[typePrefix]) return modes[typePrefix];
		return "id";
	});

	/** Per-type chip colour from project settings, with default fallback. */
	const chipColor = $derived.by((): string | null => {
		if (!typePrefix) return null;
		const colors = projectStore.projectSettings?.artifactLinks?.colors ?? {};
		return colors[typePrefix] ?? null;
	});

	/**
	 * The label to display on the chip.
	 * - If displayLabel is explicitly provided, always use it.
	 * - Otherwise: in "title" mode show the resolved title when available, fall back to ID.
	 * - In "id" mode (default) always show the raw ID.
	 */
	const chipLabel = $derived.by((): string => {
		if (displayLabel) return displayLabel;
		if (displayMode === "title" && resolved.node?.title && resolved.node.title !== resolved.targetId) {
			return resolved.node.title;
		}
		return resolved.label;
	});

	/** Whether the chip label is a title (not the raw ID). */
	const showingTitle = $derived(
		resolved.node !== null && chipLabel !== resolved.targetId,
	);

	/** Status icon component for the resolved node, or null if no status. */
	const StatusIcon = $derived(
		resolved.node?.status ? resolveIcon(statusIconName(resolved.node.status)) : null,
	);

	/** Whether the status icon should spin (active/in-progress). */
	const spinning = $derived(
		resolved.node?.status ? statusIsSpinning(resolved.node.status) : false,
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
	<TooltipRoot delayDuration={200}>
		<TooltipTrigger>
			{#snippet child({ props })}
				<button
					{...props}
					class="inline-flex items-center gap-1 whitespace-nowrap rounded border px-1.5 py-0.5 font-mono text-[11px] font-medium transition-all"
					style={chipColor
						? `background-color: color-mix(in srgb, ${chipColor} 15%, transparent); border-color: color-mix(in srgb, ${chipColor} 40%, transparent); color: ${chipColor};`
						: "background-color: rgb(6 182 212 / 0.1); border-color: rgb(6 182 212 / 0.3); color: rgb(34 211 238);"}
					onclick={handleClick}
				>
					{#if StatusIcon}
						<StatusIcon class="h-3 w-3 shrink-0 {spinning ? 'status-spin' : ''}" />
					{/if}
					{#if resolved.projectPrefix}
						<span class="opacity-50 text-[9px]">{resolved.projectPrefix}</span>
					{/if}
					{#if showingTitle}
						<span class="max-w-[200px] overflow-hidden text-ellipsis whitespace-nowrap">{chipLabel}</span>
					{:else}
						{chipLabel}
					{/if}
					<Icon name="external-link" size="xs" />
				</button>
			{/snippet}
		</TooltipTrigger>
		<TooltipContent side="top" avoidCollisions={true} collisionPadding={8} sideOffset={4}>
			{#if resolved.node}
				{@const node = resolved.node}
				<div class="space-y-1 text-xs">
					<div class="flex items-center gap-1.5">
						{#if StatusIcon}
							<StatusIcon class="h-3 w-3 shrink-0 {spinning ? 'status-spin' : ''}" />
						{/if}
						<span class="font-mono font-semibold">{node.id}</span>
						{#if node.status}
							<span class="capitalize text-muted-foreground">· {node.status}</span>
						{/if}
					</div>
					{#if node.title && node.title !== node.id}
						<p class="font-medium leading-snug">{node.title}</p>
					{/if}
					{#if descriptionSnippet}
						<p class="text-muted-foreground">{descriptionSnippet}</p>
					{/if}
				</div>
			{:else}
				<p>Navigate to {resolved.label}</p>
			{/if}
		</TooltipContent>
	</TooltipRoot>
{:else}
	<TooltipRoot>
		<TooltipTrigger>
			{#snippet child({ props })}
				<span
					{...props}
					class="inline-flex items-center gap-1 whitespace-nowrap rounded border border-warning/30 bg-warning/10 px-1.5 py-0.5 font-mono text-[11px] font-medium text-warning"
				>
					<Icon name="link-2-off" size="xs" />
					{resolved.label}
				</span>
			{/snippet}
		</TooltipTrigger>
		<TooltipContent side="top">
			<p>Not found in artifact graph: {resolved.label}</p>
		</TooltipContent>
	</TooltipRoot>
{/if}
