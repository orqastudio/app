<!-- Inline circular status indicator. Renders as an aria-hidden span with no interactive surface. -->
<script lang="ts">
	type DotSize = "xs" | "sm" | "md";
	type DotColor = "primary" | "success" | "warning" | "destructive" | "muted" | "info";

	let { size = "sm", color = "muted" }: { size?: DotSize; color?: DotColor } = $props();

	// Maps logical size names to Tailwind size utilities.
	const sizeMap: Record<DotSize, string> = {
		xs: "size-1",
		sm: "size-1.5",
		md: "size-2",
	};

	// Maps semantic color names to Tailwind background utilities.
	const colorMap: Record<DotColor, string> = {
		primary: "bg-primary",
		success: "bg-success",
		warning: "bg-warning",
		destructive: "bg-destructive",
		muted: "bg-muted-foreground/60",
		info: "bg-blue-500",
	};
</script>

<span class="inline-block shrink-0 rounded-full {sizeMap[size]} {colorMap[color]}" aria-hidden="true"></span>
