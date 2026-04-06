<script lang="ts">
	import {
		Icon,
		ScrollArea,
		Badge,
		EmptyState,
		SearchInput,
		LoadingSpinner,
		ErrorDisplay,
		Heading,
		Caption,
		Button,
		Center,
		HStack,
		Stack,
		Box,
		Text,
		Panel,
		SectionFooter,
	} from "@orqastudio/svelte-components/pure";
	import { logger } from "@orqastudio/sdk";
	import type { StoredEnforcementViolation } from "@orqastudio/types";

	const log = logger("governance");

	let {
		violations,
		loading = false,
		error = null,
		onRetry,
	}: {
		violations: StoredEnforcementViolation[];
		loading?: boolean;
		error?: string | null;
		onRetry?: () => void;
	} = $props();

	let ruleFilter = $state("");
	let actionFilter = $state<"all" | "block" | "warn">("all");

	const filtered = $derived(
		violations.filter((v) => {
			const matchesRule =
				ruleFilter.trim() === "" || v.rule_name.toLowerCase().includes(ruleFilter.toLowerCase());
			const matchesAction = actionFilter === "all" || v.action.toLowerCase() === actionFilter;
			return matchesRule && matchesAction;
		}),
	);

	const blockCount = $derived(violations.filter((v) => v.action.toLowerCase() === "block").length);
	const warnCount = $derived(violations.filter((v) => v.action.toLowerCase() === "warn").length);

	/**
	 * Format an ISO timestamp string for display in the violations list.
	 * @param iso - An ISO 8601 date string representing when the violation was recorded.
	 * @returns A localised short date-and-time string, or the original string on parse failure.
	 */
	function formatTimestamp(iso: string): string {
		try {
			const d = new Date(iso.endsWith("Z") ? iso : `${iso}Z`);
			return d.toLocaleString(undefined, {
				month: "short",
				day: "numeric",
				hour: "2-digit",
				minute: "2-digit",
			});
		} catch (err) {
			log.error("Failed to format violation timestamp", { iso, err });
			return iso;
		}
	}
</script>

<Stack gap={0} height="full">
	<!-- Header with summary counts -->
	<Panel padding="normal" border="bottom">
		<Stack gap={2}>
			<HStack justify="between">
				<Heading level={5}>Violation History</Heading>
				<HStack gap={2}>
					{#if blockCount > 0}
						<Badge variant="destructive" size="sm">
							{blockCount} blocked
						</Badge>
					{/if}
					{#if warnCount > 0}
						<Badge variant="warning" size="sm">
							{warnCount} warned
						</Badge>
					{/if}
				</HStack>
			</HStack>

			<!-- Filters -->
			<HStack gap={2}>
				<Box flex={1}>
					<SearchInput bind:value={ruleFilter} placeholder="Filter by rule name..." size="xs" />
				</Box>
				<HStack gap={1}>
					{#each ["all", "block", "warn"] as const as opt (opt)}
						<Button
							variant={actionFilter === opt ? "secondary" : "ghost"}
							size="sm"
							onclick={() => {
								actionFilter = opt;
							}}
						>
							{opt === "all" ? "All" : opt === "block" ? "Blocks" : "Warns"}
						</Button>
					{/each}
				</HStack>
			</HStack>
		</Stack>
	</Panel>

	<!-- Content -->
	<Box minHeight={0} flex={1}>
		{#if loading}
			<Center full>
				<LoadingSpinner />
			</Center>
		{:else if error}
			<Center full>
				<Panel padding="normal">
					<ErrorDisplay message={error} {onRetry} />
				</Panel>
			</Center>
		{:else if filtered.length === 0}
			<Center full>
				{#if violations.length === 0}
					<EmptyState
						icon="shield"
						title="No violations recorded"
						description="Enforcement violations will appear here when rules block or warn on tool calls."
					/>
				{:else}
					<EmptyState title="No matches" description="No violations match your current filters." />
				{/if}
			</Center>
		{:else}
			<ScrollArea full>
				<Stack gap={0}>
					{#each filtered as v (v.id)}
						<Panel padding="tight" border="bottom">
							<HStack gap={3} align="start">
								<!-- Action icon; top alignment via Box with structural flex={0} -->
								<Box flex={0}>
									{#if v.action.toLowerCase() === "block"}
										<Icon name="shield" size="sm" />
									{:else}
										<Icon name="alert-triangle" size="sm" />
									{/if}
								</Box>

								<!-- Details — min-width: 0 prevents flex child overflow -->
								<div style="min-width: 0; flex: 1; display: flex; flex-direction: column;">
									<HStack gap={2}>
										<Text variant="caption-strong" truncate>{v.rule_name}</Text>
										<Badge
											variant={v.action.toLowerCase() === "block" ? "destructive" : "warning"}
											size="xs"
										>
											{v.action.toLowerCase()}
										</Badge>
									</HStack>
									<Caption variant="caption-mono" truncate>{v.tool_name}</Caption>
									{#if v.detail}
										<Caption truncate>{v.detail}</Caption>
									{/if}
								</div>

								<!-- Timestamp -->
								<HStack gap={1} flex={0}>
									<Icon name="clock" size="xs" />
									<Caption>{formatTimestamp(v.created_at)}</Caption>
								</HStack>
							</HStack>
						</Panel>
					{/each}
				</Stack>
			</ScrollArea>
		{/if}
	</Box>

	<!-- Footer with result count -->
	{#if !loading && !error && violations.length > 0}
		<SectionFooter>
			<Caption
				>{filtered.length} of {violations.length}
				{violations.length === 1 ? "violation" : "violations"}</Caption
			>
		</SectionFooter>
	{/if}
</Stack>
