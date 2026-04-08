<!-- CollapsibleGroupHeader — CollapsibleTrigger styled as a compact group separator heading.
     Used for collapsible sections within flat lists (e.g. artifact groups, filter panels).
     Shows a chevron icon, a label, and an optional item count trailing badge.
     Depth-based left padding is handled via the optional `depth` prop. -->
<script lang="ts">
	import CollapsibleTrigger from "./collapsible-trigger.svelte";
	import { Icon } from "../icon/index.js";
	import { Caption } from "../typography/index.js";
	import { Spacer } from "../layout/index.js";

	let {
		label,
		count,
		depth = 0,
		step = 12,
	}: {
		/** Section label displayed in the header. */
		label: string;
		/** Optional item count shown at the trailing end. */
		count?: number;
		/** Tree depth for left padding (0-based). Each level adds `step` pixels. */
		depth?: number;
		/** Pixels per depth level. Defaults to 12. */
		step?: number;
	} = $props();

	const paddingLeft = $derived(depth > 0 ? `${depth * step}px` : undefined);
</script>

<CollapsibleTrigger
	class="text-muted-foreground hover:bg-accent/50 flex w-full items-center gap-2 rounded px-2 py-2 text-xs font-semibold tracking-wide uppercase"
	style={paddingLeft ? `padding-left: ${paddingLeft}` : undefined}
>
	<Icon name="chevron-right" size="xs" />
	{label}
	{#if count !== undefined}
		<Spacer />
		<Caption>{count}</Caption>
	{/if}
</CollapsibleTrigger>
