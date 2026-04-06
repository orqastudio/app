<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import {
		CardRoot,
		CardHeader,
		CardTitle,
		CardContent,
		CardAction,
	} from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import {
		Button,
		HStack,
		Stack,
		Caption,
		Code,
		CollapsibleRoot,
		CollapsibleTrigger,
		CollapsibleContent,
	} from "@orqastudio/svelte-components/pure";
	import { Panel } from "@orqastudio/svelte-components/pure";
	import { getStores, logger, fmt } from "@orqastudio/sdk";

	const log = logger("dashboard");
	import type { CliToolRunResult } from "@orqastudio/types";

	const { pluginStore } = getStores();

	let running = $state<string | null>(null);
	let lastResult = $state<CliToolRunResult | null>(null);
	let error = $state<string | null>(null);

	const hasTools = $derived(pluginStore.cliToolStatuses.length > 0);

	$effect(() => {
		void pluginStore.loadCliToolStatuses();
	});

	/**
	 * Runs a plugin CLI tool and stores the result or error.
	 * @param plugin - The plugin name that owns the tool.
	 * @param toolKey - The tool's key within the plugin.
	 */
	async function runTool(plugin: string, toolKey: string) {
		running = `${plugin}:${toolKey}`;
		error = null;
		lastResult = null;
		try {
			lastResult = await pluginStore.runCliTool(plugin, toolKey);
		} catch (err: unknown) {
			log.warn("CLI tool run failed", { plugin, toolKey, err });
			error = err instanceof Error ? err.message : String(err);
		} finally {
			running = null;
		}
	}

	/**
	 * Formats a millisecond duration as a human-readable string.
	 * @param ms - Duration in milliseconds.
	 * @returns A formatted string like "250ms" or "1.3s".
	 */
	function formatDuration(ms: number): string {
		if (ms < 1000) return `${ms}ms`;
		return `${fmt(ms / 1000, 1)}s`;
	}
</script>

{#if hasTools}
	<CardRoot>
		<CardHeader compact>
			<CardTitle>
				<HStack gap={2}>
					<Icon name="wrench" size="md" />
					Plugin CLI Tools
				</HStack>
			</CardTitle>
			<CardAction>
				<Badge variant="outline" size="xs">
					{pluginStore.cliToolStatuses.length} tool{pluginStore.cliToolStatuses.length !== 1
						? "s"
						: ""}
				</Badge>
			</CardAction>
		</CardHeader>
		<CardContent>
			<Stack gap={2}>
				{#each pluginStore.cliToolStatuses as tool (`${tool.plugin}:${tool.tool_key}`)}
					{@const isRunning = running === `${tool.plugin}:${tool.tool_key}`}
					<Panel padding="tight" border="all" rounded="md">
						<HStack justify="between">
							<HStack gap={2}>
								{#if tool.success === true}
									<Icon name="circle-check" size="sm" />
								{:else if tool.success === false}
									<Icon name="circle-x" size="sm" />
								{:else}
									<Icon name="circle-dashed" size="sm" />
								{/if}
								<Stack gap={0}>
									<Caption variant="caption-strong">{tool.label}</Caption>
									<Caption>
										{#if tool.summary}
											{tool.summary}{#if tool.last_duration_ms}
												— {formatDuration(tool.last_duration_ms)}{/if}
										{:else}
											Not run yet
										{/if}
									</Caption>
								</Stack>
							</HStack>
							<Button
								variant="ghost"
								size="sm"
								disabled={isRunning}
								onclick={() => runTool(tool.plugin, tool.tool_key)}
							>
								{#if isRunning}
									<LoadingSpinner size="sm" />
								{:else}
									Run
								{/if}
							</Button>
						</HStack>
					</Panel>
				{/each}
			</Stack>

			{#if error}
				<Caption tone="destructive">{error}</Caption>
			{/if}

			{#if lastResult && lastResult.exit_code !== 0}
				<CollapsibleRoot>
					<CollapsibleTrigger>
						<Button variant="ghost" size="sm">Last run output</Button>
					</CollapsibleTrigger>
					<CollapsibleContent>
						<Code block>{lastResult.stderr || lastResult.stdout}</Code>
					</CollapsibleContent>
				</CollapsibleRoot>
			{/if}
		</CardContent>
	</CardRoot>
{/if}
