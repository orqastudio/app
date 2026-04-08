<!-- GlowDot — compound status indicator with a translucent outer ring and a solid inner dot.
     Used in health widgets to communicate green/amber/red/empty states at a glance.
     Renders as an aria-hidden element; wrap in an accessible label if the state needs to be
     communicated to screen readers. -->
<script lang="ts">
	type Tone = "green" | "amber" | "red" | "empty";

	let { tone = "empty" }: { tone?: Tone } = $props();

	/** Maps the tone to a Tailwind background utility for both the ring and the inner dot. */
	const colorMap: Record<Tone, string> = {
		green: "bg-success",
		amber: "bg-warning",
		red: "bg-destructive",
		empty: "bg-muted-foreground/30",
	};

	const colorClass = $derived(colorMap[tone]);
</script>

<!-- Outer container: relative-positioned flex box that centres the inner dot over the ring. -->
<span aria-hidden="true" class="relative flex h-3 w-3 shrink-0 items-center justify-center">
	<span class="absolute h-3 w-3 rounded-full {colorClass} opacity-30"></span>
	<span class="h-1.5 w-1.5 rounded-full {colorClass}"></span>
</span>
