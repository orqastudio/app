<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import type { GovernanceCounts } from "$lib/types";

	interface Props {
		governance: GovernanceCounts | null;
	}

	const props: Props = $props();
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Governance</Card.Title>
		<Card.Description>Detected governance artifacts in this project</Card.Description>
	</Card.Header>
	<Card.Content>
		{#if props.governance}
			<div class="grid grid-cols-3 gap-2 text-xs">
				<div class="rounded border p-2 text-center">
					<div class="text-lg font-semibold">{props.governance.docs}</div>
					<div class="text-muted-foreground">Docs</div>
				</div>
				<div class="rounded border p-2 text-center">
					<div class="text-lg font-semibold">{props.governance.agents}</div>
					<div class="text-muted-foreground">Agents</div>
				</div>
				<div class="rounded border p-2 text-center">
					<div class="text-lg font-semibold">{props.governance.rules}</div>
					<div class="text-muted-foreground">Rules</div>
				</div>
				<div class="rounded border p-2 text-center">
					<div class="text-lg font-semibold">{props.governance.skills}</div>
					<div class="text-muted-foreground">Skills</div>
				</div>
				<div class="rounded border p-2 text-center">
					<div class="text-lg font-semibold">{props.governance.hooks}</div>
					<div class="text-muted-foreground">Hooks</div>
				</div>
				<div class="rounded border p-2 text-center">
					<div class="text-lg font-semibold">
						{props.governance.has_claude_config ? "Yes" : "No"}
					</div>
					<div class="text-muted-foreground">CLAUDE.md</div>
				</div>
			</div>
		{:else}
			<EmptyState
				icon={ShieldIcon}
				title="No governance data available"
				description="Run a project scan from Model & Scanning to detect governance artifacts."
			/>
		{/if}
	</Card.Content>
</Card.Root>
