<script lang="ts">
	import { Badge, HStack, Stack, Caption, Text, Code, Box } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";

	interface Props {
		subscriptionType: string | null | undefined;
		rateLimitTier: string | null | undefined;
		expiresAt: number | null | undefined;
		scopes: readonly string[];
	}

	const { subscriptionType, rateLimitTier, expiresAt, scopes }: Props = $props();

	/**
	 * Returns a human-readable label for a Claude subscription type.
	 * @param type - The raw subscription type string.
	 * @returns A capitalised display label.
	 */
	function formatSubscriptionType(type: string): string {
		const labels: Record<string, string> = {
			max: "Max",
			pro: "Pro",
			team: "Team",
			enterprise: "Enterprise",
			free: "Free",
		};
		return labels[type] ?? type.charAt(0).toUpperCase() + type.slice(1);
	}

	/**
	 * Strips the "default_claude_" prefix and converts underscores to spaces for display.
	 * @param tier - The raw rate limit tier identifier.
	 * @returns A human-readable tier label.
	 */
	function formatRateLimitTier(tier: string): string {
		return tier.replace(/^default_claude_/, "").replace(/_/g, " ");
	}

	/**
	 * Formats an OAuth scope string for readable display.
	 * @param scope - The raw scope string (e.g. "org:read").
	 * @returns A formatted scope string with spaces around colons and underscores replaced.
	 */
	function formatScope(scope: string): string {
		return scope.replace(/:/g, ": ").replace(/_/g, " ");
	}

	/**
	 * Returns a human-readable expiry label and whether the token has already expired.
	 * @param epochMs - The expiry time as a Unix timestamp in milliseconds.
	 * @returns An object with a display label and an expired flag.
	 */
	function formatExpiry(epochMs: number): { label: string; expired: boolean } {
		const now = Date.now();
		if (epochMs <= now) return { label: "Expired", expired: true };
		const diff = epochMs - now;
		const hours = Math.floor(diff / 3_600_000);
		const minutes = Math.floor((diff % 3_600_000) / 60_000);
		if (hours > 24) {
			const days = Math.floor(hours / 24);
			return { label: `${days}d ${hours % 24}h remaining`, expired: false };
		}
		if (hours > 0) {
			return { label: `${hours}h ${minutes}m remaining`, expired: false };
		}
		return { label: `${minutes}m remaining`, expired: false };
	}
</script>

<Separator />
<Box padding={4} rounded="lg" border background="muted">
<Stack gap={3}>
	<HStack gap={2} justify="between">
		<Text variant="body-strong">Subscription</Text>
		{#if subscriptionType}
			<Badge variant="default"><Caption variant="caption">{formatSubscriptionType(subscriptionType)}</Caption></Badge>
		{/if}
	</HStack>

	{#if rateLimitTier}
		<HStack gap={2}>
			<Caption tone="muted">Rate Limit:</Caption>
			<Code>{formatRateLimitTier(rateLimitTier)}</Code>
		</HStack>
	{/if}

	{#if expiresAt}
		{@const expiry = formatExpiry(expiresAt)}
		<HStack gap={2}>
			<Caption tone="muted">Token Expiry:</Caption>
			<Caption tone={expiry.expired ? "destructive" : undefined}>{expiry.label}</Caption>
		</HStack>
	{/if}

	{#if scopes.length > 0}
		<HStack gap={2} align="start">
			<Caption tone="muted">Scopes:</Caption>
			<HStack gap={1} wrap>
				{#each scopes as scope (scope)}
					<Badge variant="outline"><Caption variant="caption-mono">{formatScope(scope)}</Caption></Badge>
				{/each}
			</HStack>
		</HStack>
	{/if}
</Stack>
</Box>
