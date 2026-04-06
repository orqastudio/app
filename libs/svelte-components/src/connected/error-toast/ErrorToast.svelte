<script lang="ts">
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import XIcon from "@lucide/svelte/icons/x";
	import { getStores } from "@orqastudio/sdk";

	const stores = getStores();

	// ErrorToast works with the toastStore filtering for error-type entries
	const errors = $derived(stores.toastStore.toasts.filter((t) => t.type === "error"));
</script>

{#if errors.length > 0}
	<div class="fixed right-4 bottom-12 z-50 flex max-w-md flex-col gap-2">
		{#each errors as error (error.id)}
			<div
				class="border-destructive/30 bg-background/95 flex items-start gap-3 rounded-md border px-4 py-3 shadow-lg backdrop-blur-sm"
				role="alert"
			>
				<CircleAlertIcon class="text-destructive mt-0.5 h-4 w-4 flex-shrink-0" />
				<div class="min-w-0 flex-1">
					<p class="text-destructive text-xs font-medium">Error</p>
					<p class="text-muted-foreground mt-0.5 truncate text-xs">
						{error.message}
					</p>
				</div>
				<button
					class="text-muted-foreground hover:text-foreground flex-shrink-0 rounded p-0.5"
					onclick={() => stores.toastStore.remove(error.id)}
				>
					<XIcon class="h-3.5 w-3.5" />
				</button>
			</div>
		{/each}
	</div>
{/if}
