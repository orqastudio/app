<!-- PickerShell — positioned container strip for inline dropdown pickers.
     A compact horizontal bar with border-b, bg-surface, and position:relative so
     that SelectPanel children can use position:absolute to anchor their dropdown.

     The data-slot enables consumer click-outside detection without raw HTML.
     Pass a unique marker via the marker prop (e.g. "session-picker") to use as
     a data attribute for click-outside selectors in the consumer. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		marker,
		children,
	}: {
		/** Data-attribute marker for click-outside detection. Rendered as data-{marker}. */
		marker?: string;
		children?: Snippet;
	} = $props();
</script>

<!-- position:relative required for SelectPanel's position:absolute dropdown anchor. -->
<div
	data-slot="picker-shell"
	data-marker={marker}
	class="border-border bg-surface relative flex min-h-[28px] shrink-0 items-center border-b px-2"
>
	{@render children?.()}
</div>
