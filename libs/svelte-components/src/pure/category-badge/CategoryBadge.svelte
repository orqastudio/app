<!-- Category badge that renders a coloured pill for a lesson or artifact category.
     When a hex color is provided it is applied via CSS custom properties so that
     Tailwind's JIT scan is not required to generate the class at build time. -->
<script lang="ts">
	/** The category label to display. */
	let { category, color }: { category: string; color?: string } = $props();

	/** CSS variable-based inline style when a non-Tailwind hex color is supplied. */
	const inlineStyle = $derived(
		color ? `background-color: ${color}1a; color: ${color};` : undefined,
	);
</script>

<!-- Uses rounded, padding, and font-size that match the badge scale; no class= needed. -->
<span
	class={color
		? "inline-flex items-center rounded px-2 py-1 text-[11px] font-medium"
		: "bg-muted text-muted-foreground inline-flex items-center rounded px-2 py-1 text-[11px] font-medium"}
	style={inlineStyle}
>
	{category}
</span>
