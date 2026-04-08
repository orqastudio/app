<!-- SelectPanel — absolutely-positioned dropdown panel for compact inline pickers.
     Renders a z-20 floating surface directly below its trigger element. Used by
     SessionPicker and similar compact in-line selector bars.

     Position: absolute, left-anchored from the containing positioned element,
     dropping below at top:100%. z-index: 20. Width: min-width clamps the panel;
     max-height caps height with overflow-y scroll.

     The consumer is responsible for wrapping the trigger in a position:relative
     element (e.g. SectionHeader or a positioned Box) so the panel anchors correctly. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		role,
		"aria-label": ariaLabel,
		children,
	}: {
		/** ARIA role for the panel (e.g. "listbox"). */
		role?: string;
		/** Accessible label for the panel region. */
		"aria-label"?: string;
		children?: Snippet;
	} = $props();
</script>

<div
	data-slot="select-panel"
	class="border-border bg-surface-raised absolute top-full left-2 z-20 max-h-64 max-w-[480px] min-w-80 overflow-y-auto rounded-md border shadow-lg"
	{role}
	aria-label={ariaLabel}
>
	{@render children?.()}
</div>
