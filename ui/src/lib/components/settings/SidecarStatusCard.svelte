<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
	import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
	import CircleXIcon from "@lucide/svelte/icons/circle-x";
	import CircleDotIcon from "@lucide/svelte/icons/circle-dot";
	import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
	import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
	import { settingsStore } from "$lib/stores/settings.svelte";

	function sidecarStatusColor(state: string): string {
		switch (state) {
			case "connected":
				return "text-success";
			case "starting":
				return "text-warning";
			case "error":
				return "text-destructive";
			case "stopped":
			case "not_started":
			default:
				return "text-muted-foreground";
		}
	}

	function handleRestart(): void {
		settingsStore.restartSidecar();
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Provider</Card.Title>
		<Card.Description>Claude Code CLI connection and sidecar status</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-4">
		<div class="space-y-3">
			<div class="flex items-center gap-2 text-sm">
				<span class="w-32 text-muted-foreground">Sidecar Status:</span>
				<div class="flex items-center gap-1.5">
					{#if settingsStore.sidecarStatus.state === "connected"}
						<CircleCheckIcon class="h-4 w-4 text-success" />
					{:else if settingsStore.sidecarStatus.state === "starting"}
						<LoaderCircleIcon class="h-4 w-4 animate-spin text-warning" />
					{:else if settingsStore.sidecarStatus.state === "error"}
						<CircleXIcon class="h-4 w-4 text-destructive" />
					{:else}
						<CircleDotIcon class="h-4 w-4 text-muted-foreground" />
					{/if}
					<span class={sidecarStatusColor(settingsStore.sidecarStatus.state)}>
						{settingsStore.sidecarStateLabel}
					</span>
				</div>
			</div>

			{#if settingsStore.sidecarStatus.pid !== null}
				<div class="flex items-center gap-2 text-sm">
					<span class="w-32 text-muted-foreground">Process ID:</span>
					<span>{settingsStore.sidecarStatus.pid}</span>
				</div>
			{/if}

			{#if settingsStore.sidecarStatus.uptime_seconds !== null}
				<div class="flex items-center gap-2 text-sm">
					<span class="w-32 text-muted-foreground">Uptime:</span>
					<span>{Math.floor(settingsStore.sidecarStatus.uptime_seconds)}s</span>
				</div>
			{/if}

			<div class="flex items-center gap-2 text-sm">
				<span class="w-32 text-muted-foreground">CLI Detected:</span>
				{#if settingsStore.sidecarStatus.cli_detected}
					<div class="flex items-center gap-1.5">
						<CircleCheckIcon class="h-4 w-4 text-success" />
						<span>{settingsStore.sidecarStatus.cli_version ?? "Unknown version"}</span>
					</div>
				{:else}
					<div class="flex items-center gap-1.5">
						<CircleXIcon class="h-4 w-4 text-muted-foreground" />
						<span class="text-muted-foreground">Not found</span>
					</div>
				{/if}
			</div>

			{#if settingsStore.sidecarStatus.error_message}
				<div class="rounded-md border border-destructive/30 bg-destructive/10 px-3 py-2 text-sm text-destructive">
					{settingsStore.sidecarStatus.error_message}
				</div>
			{/if}
		</div>

		<Separator />

		<Button variant="outline" size="sm" onclick={handleRestart}>
			<RefreshCwIcon class="mr-1.5 h-3.5 w-3.5" />
			Restart Sidecar
		</Button>
	</Card.Content>
</Card.Root>
