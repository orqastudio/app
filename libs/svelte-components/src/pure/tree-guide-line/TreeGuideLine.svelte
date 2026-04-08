<!-- TreeGuideLine — vertical indent guide for hierarchical tree-view rows.
     Renders a 1px vertical line that provides a visual nesting cue, positioned
     at a depth-computed left offset. Used in ancestry chains and outline views
     where depth is data-driven and cannot be expressed as a closed-set token. -->
<script lang="ts">
	let {
		depth,
	}: {
		/** Tree depth level (0-based). Each level adds 12px of left margin. */
		depth: number;
	} = $props();

	const marginLeft = $derived(`${depth * 12}px`);
</script>

<div class="bg-border h-full w-px shrink-0 self-stretch" style="margin-left: {marginLeft}"></div>
