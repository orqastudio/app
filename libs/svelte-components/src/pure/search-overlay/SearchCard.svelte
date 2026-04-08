<!-- SearchCard — vertically-centred popover card for command palette search overlays.
     Positions the card in the upper third of the viewport (mt-[15vh]) with a fixed
     max-width and rounded border shadow. Contains the card frame only; content is
     supplied via the children snippet. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		children,
	}: {
		children?: Snippet;
	} = $props();
</script>

<!-- Centering wrapper: horizontally centred, positioned in upper third of viewport. -->
<div class="mx-auto mt-[15vh] w-full max-w-xl px-4">
	<!-- Card shell: popover background, rounded corners, drop shadow. -->
	<div class="border-border bg-popover overflow-hidden rounded-lg border shadow-2xl">
		{@render children?.()}
	</div>
</div>
