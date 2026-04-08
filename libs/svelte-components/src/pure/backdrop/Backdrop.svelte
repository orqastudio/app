<!-- Backdrop — fixed full-viewport overlay for command palettes and custom modals.
     Renders a translucent blurred backdrop that intercepts clicks and keyboard events.
     Forwards onclick only when the click target is the backdrop itself (not a child),
     so consumers do not need to duplicate that guard. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		zIndex = 50,
		label,
		onclick,
		onkeydown,
		children,
	}: {
		/** CSS z-index applied to the backdrop. Defaults to 50. */
		zIndex?: number;
		/** Accessible label for the dialog region (aria-label). */
		label?: string;
		/** Called when the user clicks the backdrop itself (not a child element). */
		onclick?: (e: MouseEvent) => void;
		/** Called for any keydown event that reaches the backdrop. */
		onkeydown?: (e: KeyboardEvent) => void;
		children?: Snippet;
	} = $props();

	/**
	 * Forward the click event to the onclick handler only when the target is the backdrop itself.
	 * @param e - The mouse event from the backdrop click listener.
	 */
	function handleClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			onclick?.(e);
		}
	}
</script>

<!-- tabindex="-1" ensures keyboard events are received without placing it in tab order. -->
<div
	class="bg-background/60 fixed inset-0 backdrop-blur-sm"
	style="z-index: {zIndex}"
	role="dialog"
	aria-modal="true"
	aria-label={label}
	tabindex="-1"
	onclick={handleClick}
	{onkeydown}
>
	{@render children?.()}
</div>
