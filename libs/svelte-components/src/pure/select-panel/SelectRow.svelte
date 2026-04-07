<!-- SelectRow — a single interactive row inside a SelectPanel.
     Provides the relative positioning context for the absolute context-menu panel,
     the border-b separator between rows, and optional tinted background variants
     for current (primary/8) and active (primary/15) states.

     Consumers pass data-attributes for semantic state and rely on role/aria-selected
     for accessibility. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	let {
		current = false,
		active = false,
		role,
		"aria-selected": ariaSelected,
		onclick,
		children,
	}: {
		/** When true, this is the current (live) session — shown with a primary/8 tint. */
		current?: boolean;
		/** When true, this session is actively being viewed — shown with a primary/15 tint. */
		active?: boolean;
		role?: string;
		"aria-selected"?: boolean;
		onclick?: (e: MouseEvent) => void;
		children?: Snippet;
	} = $props();
</script>

<div
	data-slot="select-row"
	class={cn(
		"border-border relative flex items-center border-b last:border-b-0",
		current && !active && "bg-primary/[0.08]",
		active && "bg-primary/[0.15]",
	)}
	role={onclick ? role || "button" : role || undefined}
	tabindex={onclick ? 0 : undefined}
	aria-selected={ariaSelected}
	{onclick}
>
	{@render children?.()}
</div>
