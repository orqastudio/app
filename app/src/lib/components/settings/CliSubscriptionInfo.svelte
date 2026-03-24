<script lang="ts">
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";

	interface Props {
		subscriptionType: string | null | undefined;
		rateLimitTier: string | null | undefined;
		expiresAt: number | null | undefined;
		scopes: string[];
	}

	const { subscriptionType, rateLimitTier, expiresAt, scopes }: Props = $props();

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

	function formatRateLimitTier(tier: string): string {
		return tier.replace(/^default_claude_/, "").replace(/_/g, " ");
	}

	function formatScope(scope: string): string {
		return scope.replace(/:/g, ": ").replace(/_/g, " ");
	}

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
<div class="rounded-lg border bg-muted/30 p-4 space-y-3">
	<div class="flex items-center justify-between">
		<span class="text-sm font-medium">Subscription</span>
		{#if subscriptionType}
			<Badge variant="default" class="text-xs capitalize">
				{formatSubscriptionType(subscriptionType)}
			</Badge>
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
					<Badge variant="outline" class="text-xs font-mono">{formatScope(scope)}</Badge>
				{/each}
			</div>
		</div>
	{/if}
</div>
