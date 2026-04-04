<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { Button, HStack, Stack, Caption, Code } from "@orqastudio/svelte-components/pure";
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
			<HStack gap={2}>
				<Icon name="loader-circle" size="md" />
				<Caption tone="muted">Checking CLI status...</Caption>
			</HStack>
		{:else if setupStore.cliInfo}
			<Stack gap={3}>
				<HStack gap={2}>
					<Caption tone="muted">Installed:</Caption>
					{#if setupStore.cliInfo.installed}
						<HStack gap={1}>
							<Icon name="circle-check" size="md" />
							<Caption>Yes</Caption>
						</HStack>
					{:else}
						<HStack gap={1}>
							<Icon name="circle-x" size="md" />
							<Caption tone="destructive">Not found</Caption>
						</HStack>
					{/if}
				</HStack>

				{#if setupStore.cliInfo.version}
					<HStack gap={2}>
						<Caption tone="muted">Version:</Caption>
						<Code>{setupStore.cliInfo.version}</Code>
					</HStack>
				{/if}

				{#if setupStore.cliInfo.path}
					<HStack gap={2}>
						<Caption tone="muted">Path:</Caption>
						<Code>{setupStore.cliInfo.path}</Code>
					</HStack>
				{/if}

				<HStack gap={2}>
					<Caption tone="muted">Authenticated:</Caption>
					{#if setupStore.cliInfo.authenticated}
						<HStack gap={1}>
							<Icon name="shield-check" size="md" />
							<Caption>Yes</Caption>
						</HStack>
					{:else}
						<HStack gap={1}>
							<Icon name="circle-x" size="md" />
							<Caption tone="warning">Not authenticated</Caption>
						</HStack>
					{/if}
				</HStack>

				{#if setupStore.cliInfo.authenticated}
					<CliSubscriptionInfo
						subscriptionType={setupStore.cliInfo.subscription_type}
						rateLimitTier={setupStore.cliInfo.rate_limit_tier}
						expiresAt={setupStore.cliInfo.expires_at}
						scopes={setupStore.cliInfo.scopes}
					/>
				{/if}
			</Stack>
		{:else}
			<Caption tone="muted">CLI status not checked yet.</Caption>
		{/if}

		<Separator />

		<HStack gap={2}>
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
		</HStack>
	</CardContent>
</CardRoot>
