<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardContent, CardAction } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { getStores, logger } from "@orqastudio/sdk";

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
		return `${(ms / 1000).toFixed(1)}s`;
	}
</script>

{#if hasTools}
<CardRoot class="gap-2">
	<CardHeader class="pb-2">
		<CardTitle class="text-sm font-semibold">
			<div class="flex items-center gap-2">
				<Icon name="wrench" size="md" />
				Plugin CLI Tools
			</div>
		</CardTitle>
		<CardAction>
			<Badge variant="outline" class="text-[10px] px-1.5 py-0">
				{pluginStore.cliToolStatuses.length} tool{pluginStore.cliToolStatuses.length !== 1 ? "s" : ""}
			</Badge>
		</CardAction>
	</CardHeader>
	<CardContent class="pt-0">
		<div class="space-y-2">
			{#each pluginStore.cliToolStatuses as tool (`${tool.plugin}:${tool.tool_key}`)}
				{@const isRunning = running === `${tool.plugin}:${tool.tool_key}`}
				<div class="flex items-center justify-between rounded border border-border px-3 py-2">
					<div class="flex items-center gap-2">
						{#if tool.success === true}
							<Icon name="circle-check" size="sm" />
						{:else if tool.success === false}
							<Icon name="circle-x" size="sm" />
						{:else}
							<Icon name="circle-dashed" size="sm" />
						{/if}
						<div>
							<p class="text-xs font-medium">{tool.label}</p>
							<p class="text-[10px] text-muted-foreground">
								{#if tool.summary}
									{tool.summary}
									{#if tool.last_duration_ms}
										 — {formatDuration(tool.last_duration_ms)}
									{/if}
								{:else}
									Not run yet
								{/if}
							</p>
						</div>
					</div>
					<Button
						variant="ghost"
						size="sm"
						class="h-7 px-2 text-xs"
						disabled={isRunning}
						onclick={() => runTool(tool.plugin, tool.tool_key)}
					>
						{#if isRunning}
							<LoadingSpinner size="sm" />
						{:else}
							Run
						{/if}
					</Button>
				</div>
			{/each}
		</div>

		{#if error}
			<p class="mt-2 text-xs text-destructive">{error}</p>
		{/if}

		{#if lastResult && lastResult.exit_code !== 0}
			<details class="mt-2">
				<summary class="cursor-pointer text-xs text-muted-foreground">
					Last run output
				</summary>
				<pre class="mt-1 max-h-32 overflow-auto rounded bg-muted p-2 text-[10px]">{lastResult.stderr || lastResult.stdout}</pre>
			</details>
		{/if}
	</CardContent>
</CardRoot>
{/if}
