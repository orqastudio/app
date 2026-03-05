<script lang="ts">
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import FileCodeIcon from "@lucide/svelte/icons/file-code";
	import BotIcon from "@lucide/svelte/icons/bot";
	import CogIcon from "@lucide/svelte/icons/cog";
	import SettingsIcon from "@lucide/svelte/icons/settings";
	import BookOpenIcon from "@lucide/svelte/icons/book-open";
	import UsersIcon from "@lucide/svelte/icons/users";
	import CheckIcon from "@lucide/svelte/icons/check";
	import XIcon from "@lucide/svelte/icons/x";
	import type { Component } from "svelte";
	import * as Card from "$lib/components/ui/card";
	import CoverageIndicator from "./CoverageIndicator.svelte";
	import type { GovernanceScanResult } from "$lib/types/governance";

	interface Props {
		scanResult: GovernanceScanResult;
	}

	const { scanResult }: Props = $props();

	const areaIconMap: Record<string, Component> = {
		rules: ShieldIcon,
		agents: BotIcon,
		skills: BookOpenIcon,
		hooks: CogIcon,
		settings: SettingsIcon,
		claude_md: FileCodeIcon,
		agents_md: UsersIcon,
	};

	const areaLabelMap: Record<string, string> = {
		rules: "Enforcement Rules",
		agents: "Agent Definitions",
		skills: "Skill Definitions",
		hooks: "Session Hooks",
		settings: "Settings & Permissions",
		claude_md: "CLAUDE.md Instructions",
		agents_md: "AGENTS.md Instructions",
	};

	function formatBytes(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		return `${(bytes / 1024).toFixed(1)} KB`;
	}

	function getAreaLabel(name: string): string {
		return areaLabelMap[name] ?? name;
	}

	function getAreaIcon(name: string): Component {
		return areaIconMap[name] ?? ShieldIcon;
	}

	const totalFiles = $derived(
		scanResult.areas.reduce((sum, area) => sum + area.files.length, 0),
	);
</script>

<div class="space-y-4">
	<CoverageIndicator areas={scanResult.areas} coverageRatio={scanResult.coverage_ratio} />

	<div class="space-y-2">
		{#each scanResult.areas as area (area.name)}
			{@const AreaIcon = getAreaIcon(area.name)}
			<Card.Root class="overflow-hidden">
				<Card.Content class="p-4">
					<div class="flex items-start gap-3">
						<div
							class="mt-0.5 flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-md {area.covered
								? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
								: 'bg-muted text-muted-foreground'}"
						>
							<AreaIcon class="h-4 w-4" />
						</div>

						<div class="min-w-0 flex-1">
							<div class="flex items-center gap-2">
								<span class="text-sm font-medium">{getAreaLabel(area.name)}</span>
								{#if area.covered}
									<CheckIcon class="h-4 w-4 text-green-600" />
								{:else}
									<XIcon class="h-4 w-4 text-muted-foreground" />
								{/if}
							</div>

							{#if area.covered && area.files.length > 0}
								<ul class="mt-1 space-y-0.5">
									{#each area.files as file (file.path)}
										<li class="flex items-center gap-2 text-xs text-muted-foreground">
											<span class="truncate font-mono">{file.path}</span>
											<span class="flex-shrink-0 text-muted-foreground/60">
												{formatBytes(file.size_bytes)}
											</span>
										</li>
									{/each}
								</ul>
							{:else if !area.covered}
								<p class="mt-0.5 text-xs text-muted-foreground">No files found</p>
							{/if}
						</div>
					</div>
				</Card.Content>
			</Card.Root>
		{/each}
	</div>

	<p class="text-xs text-muted-foreground">
		{totalFiles} governance file{totalFiles === 1 ? "" : "s"} found across all
		sources
	</p>
</div>
