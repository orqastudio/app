<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		title,
		children,
		width = 200,
	}: {
		title: string;
		children: Snippet;
		width?: number;
	} = $props();
</script>

<div class="flex flex-col overflow-hidden border-r border-border bg-muted/10" style="width: {width}px">
	<div class="flex h-10 items-center border-b border-border px-3">
		<h2 class="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
			{title}
		</h2>
	</div>
	<div class="flex-1 overflow-hidden">
		{@render children()}
	</div>
</div>
