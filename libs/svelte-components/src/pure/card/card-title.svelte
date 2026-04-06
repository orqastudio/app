<script lang="ts">
	import { cn } from "../../utils/cn.js";

	interface CardTitleProps {
		/** Title text size. */
		size?: "xs" | "sm" | "base" | "lg" | "xl";
		children?: import("svelte").Snippet;
	}

	let { size = "base", children }: CardTitleProps = $props();

	const sizeMap: Record<string, string> = {
		xs: "text-xs font-semibold",
		sm: "text-sm font-semibold",
		base: "leading-none font-semibold",
		lg: "text-lg font-semibold",
		xl: "text-xl font-semibold",
	};
</script>

<div data-slot="card-title" class={cn(sizeMap[size])}>
	{@render children?.()}
</div>
