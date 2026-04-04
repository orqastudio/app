<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardContent, CardAction } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
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
			<div class="flex items-center gap-2">
				<Icon name="wrench" size="md" />
				Plugin CLI Tools
			</div>
		</CardTitle>
		<CardAction>
			<Badge variant="outline" size="xs">
				{pluginStore.cliToolStatuses.length} tool{pluginStore.cliToolStatuses.length !== 1 ? "s" : ""}
			</Badge>
		</CardAction>
	</CardHeader>
	<CardContent>
		<div class="flex flex-col gap-2">
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
					<button
						class="flex h-7 items-center rounded px-2 text-xs hover:bg-accent disabled:opacity-50"
						disabled={isRunning}
						onclick={() => runTool(tool.plugin, tool.tool_key)}
					>
						{#if isRunning}
							<LoadingSpinner size="sm" />
						{:else}
							Run
						{/if}
					</button>
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
				<div class="mt-1 max-h-32 overflow-y-auto rounded bg-muted">
					<pre class="p-2 text-[10px]">{lastResult.stderr || lastResult.stdout}</pre>
				</div>
			</details>
		{/if}
	</CardContent>
</CardRoot>
{/if}
