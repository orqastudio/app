<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { settingsStore } = getStores();

	/**
	 * Returns a Tailwind text color class for the given sidecar connection state.
	 * @param state - The sidecar connection state string.
	 * @returns A Tailwind text color class string.
	 */
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

	/** Requests a sidecar restart through the settings store. */
	function handleRestart(): void {
		settingsStore.restartSidecar();
	}
</script>

<CardRoot>
	<CardHeader>
		<CardTitle>Provider</CardTitle>
		<CardDescription>Claude Code CLI connection and sidecar status</CardDescription>
	</CardHeader>
	<CardContent class="space-y-4">
		<div class="space-y-3">
			<div class="flex items-center gap-2 text-sm">
				<span class="w-32 text-muted-foreground">Sidecar Status:</span>
				<div class="flex items-center gap-1.5">
					{#if settingsStore.sidecarStatus.state === "connected"}
						<Icon name="circle-check" size="md" />
					{:else if settingsStore.sidecarStatus.state === "starting"}
						<Icon name="loader-circle" size="md" />
					{:else if settingsStore.sidecarStatus.state === "error"}
						<Icon name="circle-x" size="md" />
					{:else}
						<Icon name="circle-dot" size="md" />
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
						<Icon name="circle-check" size="md" />
						<span>{settingsStore.sidecarStatus.cli_version ?? "Unknown version"}</span>
					</div>
				{:else}
					<div class="flex items-center gap-1.5">
						<Icon name="circle-x" size="md" />
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
			<Icon name="refresh-cw" size="sm" />
			Restart Sidecar
		</Button>
	</CardContent>
</CardRoot>
