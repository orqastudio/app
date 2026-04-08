<!-- ColorSwatch — a rectangular color preview block with a border.
     Used in settings panels where the user can see and pick a color for an artifact
     type. The swatch is wrapped in a clickable label that activates a sr-only native
     color input. The color value comes from the plugin registry and cannot be mapped
     to the closed-set design token palette, so inline style is intentional here. -->
<script lang="ts">
	let {
		color = "#64748b",
		label,
		onchange,
	}: {
		/** Arbitrary CSS color string (hex, hsl, rgb). Comes from plugin config. */
		color?: string;
		/** Accessible label for the color input (e.g. "Pick colour for TASK"). */
		label?: string;
		/** Called with the new CSS color string when the user changes the value. */
		onchange?: (color: string) => void;
	} = $props();
</script>

<!-- The label wraps the swatch + sr-only input so clicking the swatch opens the picker. -->
<label class="flex cursor-pointer items-center gap-1" aria-label={label}>
	<span
		class="border-border inline-block h-4 w-4 shrink-0 rounded border"
		style="background-color: {color};"
	></span>
	<input
		type="color"
		class="sr-only"
		value={color}
		oninput={(e) => {
			onchange?.(e.currentTarget.value);
		}}
	/>
</label>
