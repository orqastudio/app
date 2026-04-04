<!-- Renders gate questions from a milestone or pillar artifact. Uses a bordered box to visually distinguish the question block. -->
<script lang="ts">
	import { Stack, Text, Box } from "@orqastudio/svelte-components/pure";

	let {
		questions,
	}: {
		questions: string[];
	} = $props();

	const label = $derived(questions.length === 1 ? "Gate Question" : "Gate Questions");
</script>

{#if questions.length > 0}
	<Stack gap={2}>
		<Text variant="caption">{label}</Text>
		<Box borderTop paddingX={3} paddingY={2}>
			<Stack gap={2}>
				{#each questions as question, i (i)}
					<Text variant="caption" block>"{question}"</Text>
				{/each}
			</Stack>
		</Box>
	</Stack>
{/if}
