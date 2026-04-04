<!-- Link: styled anchor element with variant-based coloring and optional external link handling. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	type Variant = "default" | "muted" | "destructive";

	let {
		href,
		external = false,
		variant = "default" as Variant,
		children,
	}: {
		href?: string;
		external?: boolean;
		variant?: Variant;
		children?: Snippet;
	} = $props();
</script>

<a
	{href}
	class={cn(
		"underline-offset-4 hover:underline",
		variant === "default" && "text-primary",
		variant === "muted" && "text-muted-foreground",
		variant === "destructive" && "text-destructive",
	)}
	target={external ? "_blank" : undefined}
	rel={external ? "noopener noreferrer" : undefined}
>
	{@render children?.()}
</a>
