<!-- TreeIndentIcon — icon wrapper with depth-computed left margin for tree rows.
     Renders a muted icon at a data-driven left offset, providing the visual
     connector between a TreeGuideLine and its indented row content. -->
<script lang="ts">
	import { Icon } from "../icon/index.js";

	let {
		name,
		depth,
	}: {
		/** Icon name to render (e.g. "corner-down-right"). */
		name: string;
		/** Tree depth level (0-based). Each level adds 12px of left margin. */
		depth: number;
	} = $props();

	const marginLeft = $derived(`${depth * 12}px`);
</script>

<span class="text-muted-foreground/50 shrink-0" style="margin-left: {marginLeft}">
	<Icon {name} size="xs" />
</span>
