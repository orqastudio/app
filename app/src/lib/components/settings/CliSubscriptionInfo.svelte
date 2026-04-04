<script lang="ts">
	import { Badge } from "@orqastudio/svelte-components/pure";
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
<div class="flex flex-col gap-3 rounded-lg border bg-muted/30 p-4">
	<div class="flex items-center justify-between">
		<span class="text-sm font-medium">Subscription</span>
		{#if subscriptionType}
			<Badge variant="default"><span class="text-xs capitalize">{formatSubscriptionType(subscriptionType)}</span></Badge>
		{/if}
	</div>

	{#if rateLimitTier}
		<div class="flex items-center gap-2 text-sm">
			<span class="w-28 text-muted-foreground">Rate Limit:</span>
			<span class="font-mono text-xs">{formatRateLimitTier(rateLimitTier)}</span>
		</div>
	{/if}

	{#if expiresAt}
		{@const expiry = formatExpiry(expiresAt)}
		<div class="flex items-center gap-2 text-sm">
			<span class="w-28 text-muted-foreground">Token Expiry:</span>
			<span class={expiry.expired ? "text-destructive font-medium" : ""}>
				{expiry.label}
			</span>
		</div>
	{/if}

	{#if scopes.length > 0}
		<div class="flex items-start gap-2 text-sm">
			<span class="w-28 shrink-0 text-muted-foreground">Scopes:</span>
			<div class="flex flex-wrap gap-1">
				{#each scopes as scope (scope)}
					<Badge variant="outline"><span class="text-xs font-mono">{formatScope(scope)}</span></Badge>
				{/each}
			</div>
		</div>
	{/if}
</div>
