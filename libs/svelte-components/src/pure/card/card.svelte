<script lang="ts">
	import { cn } from "../../utils/cn.js";
	import type { HTMLAttributes } from "svelte/elements";
	import type { Snippet } from "svelte";

	// Card extends HTMLAttributes with class and style blocked. All semantic props
	// (aria-*, data-*, role, tabindex, onclick, onkeydown, events) flow via
	// ...restProps. Typed props are the only styling interface.
	interface CardProps extends Omit<HTMLAttributes<HTMLDivElement>, "class" | "style"> {
		/** Visual variant — controls border and background colors. */
		variant?: "default" | "warning" | "destructive" | "ghost";
		/** Gap between child sections. */
		gap?: 0 | 1 | 2 | 3 | 4;
		/** Makes the card interactive (hover highlight + pointer cursor). */
		interactive?: boolean;
		/** Shows a selected-state ring using the accent color. Requires interactive=true. */
		selected?: boolean;
		/** Fill available height. */
		full?: boolean;
		children?: Snippet;
	}

	let {
		variant = "default",
		gap = 0,
		interactive = false,
		selected = false,
		full = false,
		onclick,
		children,
		...restProps
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
		interactive && "hover:bg-accent/30 cursor-pointer transition-colors",
		selected && "border-accent shadow-[0_0_0_1px_hsl(var(--accent))]",
		full && "h-full",
	)}
	{onclick}
	role={onclick ? "button" : undefined}
	tabindex={onclick ? 0 : undefined}
	{...restProps}
>
	{@render children?.()}
</div>
