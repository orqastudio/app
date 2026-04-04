<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { setupStore } = getStores();
	import CliSubscriptionInfo from "./CliSubscriptionInfo.svelte";

	interface Props {
		cliChecking: boolean;
		reauthenticating: boolean;
		onCheckCli: () => void;
		onReauthenticate: () => void;
	}

	const { cliChecking, reauthenticating, onCheckCli, onReauthenticate }: Props = $props();
</script>

<CardRoot>
	<CardHeader>
		<CardTitle>Claude CLI</CardTitle>
		<CardDescription>Claude Code CLI version and authentication status</CardDescription>
	</CardHeader>
	<CardContent>
		{#if cliChecking}
			<div class="flex items-center gap-2 text-sm">
				<Icon name="loader-circle" size="md" />
				<span class="text-muted-foreground">Checking CLI status...</span>
			</div>
		{:else if setupStore.cliInfo}
			<div class="flex flex-col gap-3">
				<div class="flex items-center gap-2 text-sm">
					<span class="w-32 text-muted-foreground">Installed:</span>
					{#if setupStore.cliInfo.installed}
						<div class="flex items-center gap-1">
							<Icon name="circle-check" size="md" />
							<span>Yes</span>
						</div>
					{:else}
						<div class="flex items-center gap-1">
							<Icon name="circle-x" size="md" />
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
						<div class="flex items-center gap-1">
							<Icon name="shield-check" size="md" />
							<span>Yes</span>
						</div>
					{:else}
						<div class="flex items-center gap-1">
							<Icon name="circle-x" size="md" />
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
			<span class="text-sm text-muted-foreground">CLI status not checked yet.</span>
		{/if}

		<Separator />

		<div class="flex items-center gap-2">
			<Button variant="outline" size="sm" onclick={onCheckCli} disabled={cliChecking}>
				<Icon name="refresh-cw" size="sm" />
				Re-check Status
			</Button>
			<Button
				variant="outline"
				size="sm"
				onclick={onReauthenticate}
				disabled={reauthenticating}
			>
				{#if reauthenticating}
					<Icon name="loader-circle" size="sm" />
					Authenticating...
				{:else}
					<Icon name="log-in" size="sm" />
					Re-authenticate
				{/if}
			</Button>
		</div>
	</CardContent>
</CardRoot>
