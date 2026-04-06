<!-- RawJson — renders pretty-printed JSON inside a scrollable, padded Code block.

Use for displaying raw event payloads, API responses, or any structured data
that must be human-readable without truncation. -->
<script lang="ts">
	import { ScrollArea } from "../scroll-area/index.js";
	import { Panel } from "../panel/index.js";
	import { Code } from "../typography/index.js";

	export interface RawJsonProps {
		/** The value to pretty-print. Accepts any JSON-serialisable type. */
		data: unknown;
	}

	let { data }: RawJsonProps = $props();

	/** Serialises data to indented JSON for display. */
	const serialised = $derived(JSON.stringify(data, null, 2));
</script>

<ScrollArea full>
	<Panel padding="tight">
		<Code block>{serialised}</Code>
	</Panel>
</ScrollArea>
