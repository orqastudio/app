<script lang="ts">
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import XIcon from "@lucide/svelte/icons/x";
	import { errorStore } from "$lib/stores/errors.svelte";

	const sourceLabels: Record<string, string> = {
		rust: "Backend",
		sidecar: "Sidecar",
		frontend: "Frontend",
	};

	function sourceLabel(source: string): string {
		return sourceLabels[source] ?? source;
	}
</script>

{#if errorStore.errors.length > 0}
	<div class="fixed bottom-12 right-4 z-50 flex max-w-md flex-col gap-2">
		{#each errorStore.errors as error (error.id)}
			<div
				class="flex items-start gap-3 rounded-md border border-destructive/30 bg-background/95 px-4 py-3 shadow-lg backdrop-blur-sm"
				role="alert"
			>
				<CircleAlertIcon class="mt-0.5 h-4 w-4 flex-shrink-0 text-destructive" />
				<div class="min-w-0 flex-1">
					<p class="text-xs font-medium text-destructive">
						{sourceLabel(error.source)}
					</p>
					<p class="mt-0.5 truncate text-xs text-muted-foreground">
						{error.message}
					</p>
				</div>
				<button
					class="flex-shrink-0 rounded p-0.5 text-muted-foreground hover:text-foreground"
					onclick={() => errorStore.dismiss(error.id)}
				>
					<XIcon class="h-3.5 w-3.5" />
				</button>
			</div>
		{/each}
	</div>
{/if}
