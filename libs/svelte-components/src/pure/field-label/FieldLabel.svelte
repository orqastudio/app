<!-- FieldLabel — fixed-width label for two-column key-value rows.
     Renders a muted, capitalized text column at a standard 7rem width so that
     a list of field rows aligns cleanly without needing ad-hoc w-[7rem] spans. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		children,
	}: {
		children?: Snippet;
	} = $props();
</script>

<span class="text-muted-foreground w-[7rem] shrink-0 text-xs font-medium capitalize">
	{@render children?.()}
</span>
