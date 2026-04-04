<script lang="ts">
	import { cn } from "../../utils/cn.js";

	interface CardProps {
		/** Visual variant — controls border and background colors. */
		variant?: "default" | "warning" | "destructive" | "ghost";
		/** Gap between child sections. */
		gap?: 0 | 1 | 2 | 3 | 4;
		/** Makes the card interactive (hover highlight + pointer cursor). */
		interactive?: boolean;
		/** Fill available height. */
		full?: boolean;
		/** Click handler for interactive cards. */
		onclick?: (e: MouseEvent) => void;
		/** Svelte children snippet. */
		children?: import("svelte").Snippet;
	}

	let {
		variant = "default",
		gap = 0,
		interactive = false,
		full = false,
		onclick,
		children,
	}: CardProps = $props();

	const gapMap: Record<number, string> = {
		0: "gap-0",
		1: "gap-1",
		2: "gap-2",
		3: "gap-3",
		4: "gap-4",
	};

	const variantMap: Record<string, string> = {
		default: "bg-card text-card-foreground border shadow-sm",
		warning: "border-warning/40 bg-warning/5 text-card-foreground",
		destructive: "border-destructive/50 bg-destructive/10 text-card-foreground",
		ghost: "border-border bg-transparent shadow-none text-card-foreground",
	};
</script>

<div
	data-slot="card"
	class={cn(
		"flex flex-col rounded-xl",
		variantMap[variant],
		gapMap[gap],
		interactive && "cursor-pointer transition-colors hover:bg-accent/30",
		full && "h-full",
	)}
	{onclick}
	role={onclick ? "button" : undefined}
	tabindex={onclick ? 0 : undefined}
>
	{@render children?.()}
</div>
