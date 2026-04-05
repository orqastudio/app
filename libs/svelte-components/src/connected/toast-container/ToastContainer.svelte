<script lang="ts">
	import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
	import InfoIcon from "@lucide/svelte/icons/info";
	import XIcon from "@lucide/svelte/icons/x";
	import { getStores } from "@orqastudio/sdk";

	type ToastType = "success" | "error" | "warning" | "info";

	const stores = getStores();

	const iconMap: Record<ToastType, typeof CheckCircleIcon> = {
		success: CheckCircleIcon,
		error: CircleAlertIcon,
		warning: TriangleAlertIcon,
		info: InfoIcon,
	};

	const styleMap: Record<ToastType, { border: string; icon: string; label: string }> = {
		success: { border: "border-success/30", icon: "text-success", label: "text-success" },
		error: { border: "border-destructive/30", icon: "text-destructive", label: "text-destructive" },
		warning: { border: "border-warning/30", icon: "text-warning", label: "text-warning" },
		info: { border: "border-primary/30", icon: "text-primary", label: "text-primary" },
	};

	const typeLabels: Record<ToastType, string> = {
		success: "Success",
		error: "Error",
		warning: "Warning",
		info: "Info",
	};
</script>

{#if stores.toastStore.toasts.length > 0}
	<div class="fixed top-4 left-1/2 z-50 flex -translate-x-1/2 flex-col gap-2">
		{#each stores.toastStore.toasts as entry (entry.id)}
			{@const styles = styleMap[entry.type as ToastType]}
			{@const Icon = iconMap[entry.type as ToastType]}
			<div
				class="toast-enter flex min-w-[300px] items-start gap-3 rounded-md border {styles.border} bg-background/95 px-4 py-3 shadow-lg backdrop-blur-sm"
				role="status"
				aria-live="polite"
			>
				<Icon class="mt-0.5 h-4 w-4 flex-shrink-0 {styles.icon}" />
				<div class="min-w-0 flex-1">
					<p class="text-xs font-medium {styles.label}">
						{typeLabels[entry.type as ToastType]}
					</p>
					<p class="mt-0.5 text-xs text-muted-foreground">
						{entry.message}
					</p>
				</div>
				<button
					class="flex-shrink-0 rounded p-0.5 text-muted-foreground hover:text-foreground"
					onclick={() => stores.toastStore.remove(entry.id)}
				>
					<XIcon class="h-3.5 w-3.5" />
				</button>
			</div>
		{/each}
	</div>
{/if}

<style>
	@keyframes toast-slide-in {
		from { opacity: 0; transform: translateY(-0.75rem); }
		to { opacity: 1; transform: translateY(0); }
	}
	.toast-enter {
		animation: toast-slide-in 200ms ease-out forwards;
	}
</style>
