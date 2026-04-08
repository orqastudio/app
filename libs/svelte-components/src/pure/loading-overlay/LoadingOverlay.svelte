<!-- LoadingOverlay — absolute-position overlay with blurred semi-transparent background.
     Covers its nearest positioned ancestor and centers its children vertically and
     horizontally. Used to show a loading spinner while content renders in place. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let { children }: { children?: Snippet } = $props();
</script>

<div
	class="absolute inset-0 flex flex-col items-center justify-center gap-4 backdrop-blur-sm"
	style="background: hsl(var(--background) / 0.6);"
>
	{@render children?.()}
</div>
