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

<div
	class="border-border bg-muted/10 flex flex-col overflow-hidden border-r"
	style="width: {width}px"
>
	<div class="border-border flex h-10 items-center border-b px-3">
		<h2 class="text-muted-foreground text-xs font-semibold tracking-wide uppercase">
			{title}
		</h2>
	</div>
	<div class="flex-1 overflow-hidden">
		{@render children()}
	</div>
</div>
