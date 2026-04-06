<!-- ContextTable — renders a key-value table of event structured fields.

Each entry is a label/value pair. When onValueClick is provided, values become
clickable buttons — used for navigating to a related view (e.g. Trace) via a
correlation_id. -->
<script lang="ts">
	import { Stack } from "../layout/index.js";
	import { HStack } from "../layout/index.js";
	import { Caption } from "../typography/index.js";
	import { Code } from "../typography/index.js";

	/** A single key-value entry in the context table. */
	export interface ContextEntry {
		key: string;
		value: string;
		/** When true, the value may be activated by onValueClick. */
		copyable?: boolean;
	}

	export interface ContextTableProps {
		/** Ordered list of entries to display. */
		entries: ContextEntry[];
		/** Called when a value cell is clicked; receives the key and value. */
		onValueClick?: (key: string, value: string) => void;
	}

	let { entries, onValueClick }: ContextTableProps = $props();

	/**
	 * Handles click on a value cell. Only fires when onValueClick is provided,
	 * enabling navigation to a correlated view (e.g. Trace by correlation_id).
	 * @param key - the entry key
	 * @param value - the entry value
	 */
	function handleValueClick(key: string, value: string) {
		onValueClick?.(key, value);
	}
</script>

<Stack gap={0}>
	{#each entries as entry (entry.key)}
		<HStack gap={2} full justify="between" align="start">
			<Caption variant="caption" tone="muted">{entry.key}</Caption>
			{#if onValueClick}
				<button
					class="cursor-pointer text-left hover:underline"
					onclick={() => handleValueClick(entry.key, entry.value)}
					type="button"
				>
					<Code>{entry.value}</Code>
				</button>
			{:else}
				<Code>{entry.value}</Code>
			{/if}
		</HStack>
	{/each}
</Stack>
