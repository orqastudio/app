<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import type { ToolCallState } from "@orqastudio/sdk";
	import { getActivityPhase, getEphemeralLabel } from "$lib/utils/tool-display";
	import {
		Caption,
		Text,
		StreamingDots,
		Stack,
		HStack,
		Panel,
	} from "@orqastudio/svelte-components/pure";

	let {
		hasContent = false,
		toolCalls = [],
	}: {
		hasContent?: boolean;
		toolCalls?: ToolCallState[];
	} = $props();

	const WAITING_PHRASES = [
		"Thinking",
		"Orqing",
		"Pondering",
		"Contemplating",
		"Brewing ideas",
		"Consulting the oracle",
		"Crunching thoughts",
		"Summoning knowledge",
		"Warming up neurons",
		"Channeling wisdom",
		"Connecting dots",
		"Assembling thoughts",
	];

	let phraseIndex = $state(Math.floor(Math.random() * WAITING_PHRASES.length));
	let intervalId: ReturnType<typeof setInterval> | null = null;

	onMount(() => {
		intervalId = setInterval(() => {
			let next: number;
			do {
				next = Math.floor(Math.random() * WAITING_PHRASES.length);
			} while (next === phraseIndex && WAITING_PHRASES.length > 1);
			phraseIndex = next;
		}, 10_000);
	});

	onDestroy(() => {
		if (intervalId !== null) clearInterval(intervalId);
	});

	// Determine the current mode
	const hasActiveTools = $derived(toolCalls.length > 0);
	const runningTool = $derived(toolCalls.find((t) => !t.isComplete));
	const lastCompletedTool = $derived(toolCalls.length > 0 ? toolCalls[toolCalls.length - 1] : null);

	// Activity phase from the most recent tool
	const currentPhase = $derived(
		runningTool
			? getActivityPhase(runningTool.toolName)
			: lastCompletedTool
				? getActivityPhase(lastCompletedTool.toolName)
				: "Working",
	);

	// Ephemeral label for the currently running tool
	const ephemeralText = $derived(
		runningTool ? getEphemeralLabel(runningTool.toolName, runningTool.input) : null,
	);

	// The main status label — tools take priority
	const statusLabel = $derived.by(() => {
		if (hasActiveTools) return currentPhase;
		return WAITING_PHRASES[phraseIndex];
	});

	// Hide entirely when content is streaming — the user can see tokens arriving
	const visible = $derived(!hasContent || hasActiveTools);
</script>

{#if visible}
	<Panel padding="tight">
		<Stack gap={1}>
			<HStack gap={2}>
				<StreamingDots />
				<Text variant="body-strong-muted">{statusLabel}...</Text>
				{#if hasActiveTools}
					<Text variant="tabular">
						({toolCalls.filter((t) => t.isComplete).length}/{toolCalls.length} tools)
					</Text>
				{/if}
			</HStack>
			{#if ephemeralText}
				<Caption truncate>{ephemeralText}</Caption>
			{/if}
		</Stack>
	</Panel>
{/if}
