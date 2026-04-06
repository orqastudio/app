<!-- Renders a list of acceptance criteria for a task artifact, using check icons when the task is completed. -->
<script lang="ts">
	import {
		Icon,
		CardRoot,
		CardContent,
		Stack,
		HStack,
		Text,
	} from "@orqastudio/svelte-components/pure";

	let {
		criteria,
		status = "",
	}: {
		criteria: string[];
		status?: string;
	} = $props();

	const isDone = $derived(status === "completed");
</script>

{#if criteria.length > 0}
	<CardRoot>
		<CardContent>
			<Stack gap={2}>
				<Text variant="overline-muted" block>Acceptance Criteria</Text>
				<Stack gap={1.5}>
					{#each criteria as item, i (i)}
						<HStack gap={2} align="start">
							{#if isDone}
								<Icon name="square-check" size="sm" />
							{:else}
								<Icon name="square" size="sm" />
							{/if}
							<Text variant={isDone ? "body-muted" : "body"}>{item}</Text>
						</HStack>
					{/each}
				</Stack>
			</Stack>
		</CardContent>
	</CardRoot>
{/if}
