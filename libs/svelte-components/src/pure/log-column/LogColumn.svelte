<!-- LogColumn — fixed-width column cell for virtualised log table rows.
     Encapsulates the fixed-width/flex-shrink-0/truncation pattern used in
     LogRow column spans. The "fill" variant expands to fill remaining row width.

     All column variants use 11px caption-sized text. The timestamp variant
     applies monospace tabular-nums styling for aligned time display.
     This component only renders an inline span — callers compose it inside
     a Button (the clickable row) within a LogRowShell (the positioned container). -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Width map for fixed-width column variants (in pixels, expressed as Tailwind
	// arbitrary value classes). These match the LogRow column header widths in LogTable.
	const variantClasses: Record<string, string> = {
		// 90px: timestamp column. Monospace, tabular-nums, muted.
		timestamp: "w-[90px] shrink-0 font-mono text-[11px] text-muted-foreground tabular-nums",
		// 52px: level badge cell. Badge (42px) + right-margin (8px) + padding.
		badge: "w-[52px] shrink-0 mr-2 flex items-center",
		// 80px: source column. Muted caption.
		source:
			"w-[80px] shrink-0 overflow-hidden text-ellipsis whitespace-nowrap text-[11px] text-muted-foreground",
		// 120px: category column.
		category:
			"w-[120px] shrink-0 overflow-hidden text-ellipsis whitespace-nowrap text-[11px] text-muted-foreground",
		// Fills remaining row width; used for the message column.
		fill: "min-w-0 flex-1 overflow-hidden text-ellipsis whitespace-nowrap text-[11px] text-foreground",
	};

	let {
		variant,
		children,
	}: {
		/** Column type. Controls width, font style, and overflow behavior. */
		variant: "timestamp" | "badge" | "source" | "category" | "fill";
		children?: Snippet;
	} = $props();
</script>

<span data-slot="log-column" class={cn(variantClasses[variant])}>
	{@render children?.()}
</span>
