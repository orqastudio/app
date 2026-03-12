<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
	import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
	import CircleXIcon from "@lucide/svelte/icons/circle-x";
	import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
	import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
	import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
	import LogInIcon from "@lucide/svelte/icons/log-in";
	import { setupStore } from "$lib/stores/setup.svelte";
	import CliSubscriptionInfo from "./CliSubscriptionInfo.svelte";

	interface Props {
		cliChecking: boolean;
		reauthenticating: boolean;
		onCheckCli: () => void;
		onReauthenticate: () => void;
	}

	const { cliChecking, reauthenticating, onCheckCli, onReauthenticate }: Props = $props();
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Claude CLI</Card.Title>
		<Card.Description>Claude Code CLI version and authentication status</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-4">
		{#if cliChecking}
			<div class="flex items-center gap-2 text-sm">
				<LoaderCircleIcon class="h-4 w-4 animate-spin text-muted-foreground" />
				<span class="text-muted-foreground">Checking CLI status...</span>
			</div>
		{:else if setupStore.cliInfo}
			<div class="space-y-3">
				<div class="flex items-center gap-2 text-sm">
					<span class="w-32 text-muted-foreground">Installed:</span>
					{#if setupStore.cliInfo.installed}
						<div class="flex items-center gap-1.5">
							<CircleCheckIcon class="h-4 w-4 text-success" />
							<span>Yes</span>
						</div>
					{:else}
						<div class="flex items-center gap-1.5">
							<CircleXIcon class="h-4 w-4 text-destructive" />
							<span class="text-destructive">Not found</span>
						</div>
					{/if}
				</div>

				{#if setupStore.cliInfo.version}
					<div class="flex items-center gap-2 text-sm">
						<span class="w-32 text-muted-foreground">Version:</span>
						<span class="font-mono text-xs">{setupStore.cliInfo.version}</span>
					</div>
				{/if}

				{#if setupStore.cliInfo.path}
					<div class="flex items-center gap-2 text-sm">
						<span class="w-32 text-muted-foreground">Path:</span>
						<span class="font-mono text-xs">{setupStore.cliInfo.path}</span>
					</div>
				{/if}

				<div class="flex items-center gap-2 text-sm">
					<span class="w-32 text-muted-foreground">Authenticated:</span>
					{#if setupStore.cliInfo.authenticated}
						<div class="flex items-center gap-1.5">
							<ShieldCheckIcon class="h-4 w-4 text-success" />
							<span>Yes</span>
						</div>
					{:else}
						<div class="flex items-center gap-1.5">
							<CircleXIcon class="h-4 w-4 text-warning" />
							<span class="text-warning">Not authenticated</span>
						</div>
					{/if}
				</div>

				{#if setupStore.cliInfo.authenticated}
					<CliSubscriptionInfo
						subscriptionType={setupStore.cliInfo.subscription_type}
						rateLimitTier={setupStore.cliInfo.rate_limit_tier}
						expiresAt={setupStore.cliInfo.expires_at}
						scopes={setupStore.cliInfo.scopes}
					/>
				{/if}
			</div>
		{:else}
			<p class="text-sm text-muted-foreground">CLI status not checked yet.</p>
		{/if}

		<Separator />

		<div class="flex gap-2">
			<Button variant="outline" size="sm" onclick={onCheckCli} disabled={cliChecking}>
				<RefreshCwIcon class="mr-1.5 h-3.5 w-3.5" />
				Re-check Status
			</Button>
			<Button
				variant="outline"
				size="sm"
				onclick={onReauthenticate}
				disabled={reauthenticating}
			>
				{#if reauthenticating}
					<LoaderCircleIcon class="mr-1.5 h-3.5 w-3.5 animate-spin" />
					Authenticating...
				{:else}
					<LogInIcon class="mr-1.5 h-3.5 w-3.5" />
					Re-authenticate
				{/if}
			</Button>
		</div>
	</Card.Content>
</Card.Root>
