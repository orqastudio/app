<script lang="ts">
	import { cn } from "../../utils/cn.js";

	interface CardDescriptionProps {
		/** Text size. */
		size?: "xs" | "sm";
		children?: import("svelte").Snippet;
	}

	let {
		size = "sm",
		children,
	}: CardDescriptionProps = $props();
</script>

<p
	data-slot="card-description"
	class={cn("text-muted-foreground", size === "xs" ? "text-xs" : "text-sm")}
>
	{@render children?.()}
</p>
