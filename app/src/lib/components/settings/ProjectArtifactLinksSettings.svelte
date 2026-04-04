<script lang="ts">
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import type { ProjectSettings, ArtifactLinksConfig, ArtifactLinkDisplayMode } from "@orqastudio/types";

	interface Props {
		settings: ProjectSettings;
		onSave: (settings: ProjectSettings) => void;
	}

	const props: Props = $props();

	// Effective colors are the persisted values from project settings only.
	// Defaults come from the plugin registry, not hardcoded constants.
	const effectiveColors = $derived.by((): Record<string, string> => {
		return { ...(props.settings.artifactLinks?.colors ?? {}) };
	});

	const effectiveDisplayModes = $derived.by((): Record<string, ArtifactLinkDisplayMode> => {
		return props.settings.artifactLinks?.displayModes ?? {};
	});

	/** All type prefixes, in display order — from persisted settings. */
	const prefixes = $derived(Object.keys(effectiveColors));

	function getDisplayMode(prefix: string): ArtifactLinkDisplayMode {
		return effectiveDisplayModes[prefix] ?? "id";
	}

	function buildConfig(): ArtifactLinksConfig {
		return {
			displayModes: effectiveDisplayModes,
			colors: effectiveColors,
		};
	}

	function handleDisplayModeChange(prefix: string, mode: ArtifactLinkDisplayMode) {
		const displayModes = { ...effectiveDisplayModes, [prefix]: mode };
		props.onSave({
			...props.settings,
			artifactLinks: { ...buildConfig(), displayModes },
		});
	}

	function handleColorChange(prefix: string, color: string) {
		const colors = { ...effectiveColors, [prefix]: color };
		props.onSave({
			...props.settings,
			artifactLinks: { ...buildConfig(), colors },
		});
	}

	function resetColor(prefix: string) {
		const colors = { ...effectiveColors };
		// Remove color override — no hardcoded default to restore to.
		delete colors[prefix];
		props.onSave({
			...props.settings,
			artifactLinks: { ...buildConfig(), colors },
		});
	}
</script>

<CardRoot>
	<CardHeader>
		<CardTitle>Artifact Links</CardTitle>
		<CardDescription>Control how artifact link chips are displayed across the app</CardDescription>
	</CardHeader>
	<CardContent>
		<!-- Column headers -->
		<div class="grid grid-cols-[6rem_1fr_8rem] items-center gap-x-4 px-1">
			<span class="text-xs font-medium text-muted-foreground">Type</span>
			<span class="text-xs font-medium text-muted-foreground">Display</span>
			<span class="text-xs font-medium text-muted-foreground">Colour</span>
		</div>

		<Separator />

		<!-- Per-type rows -->
		<div class="space-y-1.5">
			{#each prefixes as prefix (prefix)}
				{@const color = effectiveColors[prefix] ?? "#64748b"}
				{@const isDefault = false}
				{@const mode = getDisplayMode(prefix)}

				<div class="grid grid-cols-[6rem_1fr_8rem] items-center gap-x-4">
					<!-- Type label -->
					<span class="font-mono text-xs font-semibold">{prefix}</span>

					<!-- Display mode toggle -->
					<div class="flex gap-1.5">
						<button
							class="flex h-6 items-center rounded px-2 text-xs {mode === 'id' ? 'bg-primary text-primary-foreground' : 'border border-border hover:bg-accent'}"
							onclick={() => handleDisplayModeChange(prefix, "id")}
						>
							ID
						</button>
						<button
							class="flex h-6 items-center rounded px-2 text-xs {mode === 'title' ? 'bg-primary text-primary-foreground' : 'border border-border hover:bg-accent'}"
							onclick={() => handleDisplayModeChange(prefix, "title")}
						>
							Title
						</button>
					</div>

					<!-- Colour swatch + native picker + reset -->
					<div class="flex items-center gap-1.5">
						<label class="flex cursor-pointer items-center gap-1" aria-label="Pick colour for {prefix}">
							<span
								class="inline-block h-4 w-4 shrink-0 rounded border border-border"
								style="background-color: {color};"
							></span>
							<input
								type="color"
								class="sr-only"
								value={color}
								oninput={(e) => {
									const target = e.currentTarget;
									handleColorChange(prefix, target.value);
								}}
							/>
						</label>
						{#if !isDefault}
							<button
								class="h-auto px-0 text-[10px] text-muted-foreground hover:text-foreground"
								aria-label="Reset to default"
								onclick={() => resetColor(prefix)}
							>
								↩
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</CardContent>
</CardRoot>
