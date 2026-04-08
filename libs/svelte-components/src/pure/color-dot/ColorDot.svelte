<!-- ColorDot — inline circular status indicator with an arbitrary CSS color value.
     Used in roadmap and kanban views where stage colors come from the plugin registry
     and cannot be mapped to the closed-set design token palette.
     Renders as an aria-hidden span with no interactive surface. -->
<script lang="ts">
	let {
		color = "#94a3b8",
	}: {
		/** Arbitrary CSS color string (hex, hsl, rgb). Comes from the plugin manifest. */
		color?: string;
	} = $props();
</script>

<span
	aria-hidden="true"
	class="mt-1 inline-block h-2 w-2 shrink-0 rounded-full"
	style="background-color: {color};"
></span>
