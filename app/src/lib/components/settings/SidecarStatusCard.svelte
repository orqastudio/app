<script lang="ts">
	import {
		Icon,
		CardRoot,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
	} from "@orqastudio/svelte-components/pure";
	import { Button, HStack, Stack, Caption } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { settingsStore } = getStores();

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
	<CardContent>
		<Stack gap={3}>
			<HStack gap={2}>
				<Caption tone="muted">Sidecar Status:</Caption>
				<HStack gap={1}>
					{#if settingsStore.sidecarStatus.state === "connected"}
						<Icon name="circle-check" size="md" />
					{:else if settingsStore.sidecarStatus.state === "starting"}
						<Icon name="loader-circle" size="md" />
					{:else if settingsStore.sidecarStatus.state === "error"}
						<Icon name="circle-x" size="md" />
					{:else}
						<Icon name="circle-dot" size="md" />
					{/if}
					<Caption>{settingsStore.sidecarStateLabel}</Caption>
				</HStack>
			</HStack>

			{#if settingsStore.sidecarStatus.pid !== null}
				<HStack gap={2}>
					<Caption tone="muted">Process ID:</Caption>
					<Caption>{settingsStore.sidecarStatus.pid}</Caption>
				</HStack>
			{/if}

			{#if settingsStore.sidecarStatus.uptime_seconds !== null}
				<HStack gap={2}>
					<Caption tone="muted">Uptime:</Caption>
					<Caption>{Math.floor(settingsStore.sidecarStatus.uptime_seconds)}s</Caption>
				</HStack>
			{/if}

			<HStack gap={2}>
				<Caption tone="muted">CLI Detected:</Caption>
				{#if settingsStore.sidecarStatus.cli_detected}
					<HStack gap={1}>
						<Icon name="circle-check" size="md" />
						<Caption>{settingsStore.sidecarStatus.cli_version ?? "Unknown version"}</Caption>
					</HStack>
				{:else}
					<HStack gap={1}>
						<Icon name="circle-x" size="md" />
						<Caption tone="muted">Not found</Caption>
					</HStack>
				{/if}
			</HStack>

			{#if settingsStore.sidecarStatus.error_message}
				<!-- Error message box: destructive/30 border and destructive/10 bg are not in Box typed props — keep as raw div -->
				<div
					class="border-destructive/30 bg-destructive/10 text-destructive rounded-md border px-3 py-2 text-sm"
				>
					{settingsStore.sidecarStatus.error_message}
				</div>
			{/if}
		</Stack>

		<Separator />

		<Button variant="outline" size="sm" onclick={handleRestart}>
			<Icon name="refresh-cw" size="sm" />
			Restart Sidecar
		</Button>
	</CardContent>
</CardRoot>
