<script lang="ts">
	import { Badge, type BadgeVariant } from "../badge/index.js";
	import type { Snippet } from "svelte";

	let {
		variant = "default",
		children,
	}: {
		variant?: BadgeVariant;
		children: Snippet;
	} = $props();
</script>

<Badge {variant} class="text-[11px] leading-none">{@render children()}</Badge>
