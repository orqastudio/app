<!-- StackFrameList renders an ordered list of resolved stack trace frames.
     The top frame (index 0) is emphasized. Clicking any frame copies file:line
     to the clipboard and shows a transient "Copied!" tooltip. -->
<script lang="ts">
	import { Stack } from "../layout/index.js";
	import { HStack } from "../layout/index.js";
	import { Caption } from "../typography/index.js";
	import { Text } from "../typography/index.js";
	import { Code } from "../typography/index.js";
	import SimpleTooltip from "../tooltip/SimpleTooltip.svelte";

	/** A single resolved stack frame. All fields except `file` are optional. */
	export interface StackFrame {
		file: string;
		line?: number;
		col?: number;
		function?: string;
		raw?: string;
	}

	export interface StackFrameListProps {
		frames: StackFrame[];
	}

	let { frames }: StackFrameListProps = $props();

	/** Tracks which frame index is currently showing the "Copied!" tooltip. */
	let copiedIndex = $state<number | null>(null);

	/**
	 * Copies `file:line` for the given frame to the clipboard and shows a
	 * transient "Copied!" tooltip for 1.5 seconds.
	 * @param frame - The stack frame to copy location from.
	 * @param index - The frame index, used for tooltip tracking.
	 */
	async function copyFrame(frame: StackFrame, index: number): Promise<void> {
		const text = frame.line != null ? `${frame.file}:${frame.line}` : frame.file;
		await navigator.clipboard.writeText(text);
		copiedIndex = index;
		setTimeout(() => {
			copiedIndex = null;
		}, 1500);
	}

	/**
	 * Builds the file location string shown in the Caption.
	 * Format: file:line:col, file:line, or file depending on available fields.
	 * @param frame - The stack frame to extract location from.
	 * @returns Formatted file:line:col string.
	 */
	function locationLabel(frame: StackFrame): string {
		if (frame.line != null && frame.col != null) {
			return `${frame.file}:${frame.line}:${frame.col}`;
		}
		if (frame.line != null) {
			return `${frame.file}:${frame.line}`;
		}
		return frame.file;
	}

	/**
	 * Resolves the display name for a frame's function slot.
	 * Falls back to the raw string if no structured name is available.
	 * @param frame - The stack frame to extract function name from.
	 * @returns The function name, raw frame, or "(anonymous)".
	 */
	function functionLabel(frame: StackFrame): string {
		return frame.function ?? frame.raw ?? "(anonymous)";
	}
</script>

{#if frames.length === 0}
	<Caption>No stack frames available</Caption>
{:else}
	<Stack gap={0}>
		{#each frames as frame, index (frame.file + ":" + (frame.line ?? index))}
			<SimpleTooltip side="top" delayDuration={0}>
				{#snippet trigger({ props })}
					<HStack
						{...props}
						gap={2}
						justify="between"
						full
						role="button"
						tabindex={0}
						onclick={() => copyFrame(frame, index)}
						onkeydown={(e) => e.key === "Enter" && copyFrame(frame, index)}
					>
						{#if index === 0}
							<Text variant="body-strong">{functionLabel(frame)}</Text>
						{:else}
							<Code>{functionLabel(frame)}</Code>
						{/if}
						<Caption variant="caption-mono" truncate>{locationLabel(frame)}</Caption>
					</HStack>
				{/snippet}
				{copiedIndex === index ? "Copied!" : "Click to copy"}
			</SimpleTooltip>
		{/each}
	</Stack>
{/if}
