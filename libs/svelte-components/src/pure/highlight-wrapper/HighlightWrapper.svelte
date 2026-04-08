<!-- HighlightWrapper — scrollable container for svelte-highlight code blocks.
     Overrides svelte-highlight's default code/pre styles so they inherit the
     parent's background, removes excess margins, and reduces padding to match
     the design system's compact code block appearance. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let { children }: { children?: Snippet } = $props();
</script>

<div
	class="overflow-x-auto text-sm [&_code]:!bg-transparent [&_pre]:!my-0 [&_pre]:!bg-transparent [&_pre]:!p-1"
>
	{@render children?.()}
</div>
