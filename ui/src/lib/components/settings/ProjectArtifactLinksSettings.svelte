<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Separator } from "$lib/components/ui/separator";
	import type { ProjectSettings, ArtifactLinksConfig, ArtifactLinkDisplayMode } from "$lib/types";
	import { DEFAULT_ARTIFACT_LINK_COLORS } from "$lib/types";

	interface Props {
		settings: ProjectSettings;
		onSave: (settings: ProjectSettings) => void;
	}

	const props: Props = $props();

	// Resolve effective config — merge defaults with persisted values.
	const effectiveColors = $derived.by((): Record<string, string> => {
		return { ...DEFAULT_ARTIFACT_LINK_COLORS, ...(props.settings.artifactLinks?.colors ?? {}) };
	});

	const effectiveDisplayModes = $derived.by((): Record<string, ArtifactLinkDisplayMode> => {
		return props.settings.artifactLinks?.displayModes ?? {};
	});

	/** All type prefixes, in display order. */
	const prefixes = Object.keys(DEFAULT_ARTIFACT_LINK_COLORS);

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
		const defaultColor = DEFAULT_ARTIFACT_LINK_COLORS[prefix];
		if (defaultColor) {
			colors[prefix] = defaultColor;
		} else {
			delete colors[prefix];
		}
		props.onSave({
			...props.settings,
			artifactLinks: { ...buildConfig(), colors },
		});
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Artifact Links</Card.Title>
		<Card.Description>Control how artifact link chips are displayed across the app</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-3">
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
				{@const color = effectiveColors[prefix] ?? DEFAULT_ARTIFACT_LINK_COLORS[prefix] ?? "#64748b"}
				{@const isDefault = color === DEFAULT_ARTIFACT_LINK_COLORS[prefix]}
				{@const mode = getDisplayMode(prefix)}

				<div class="grid grid-cols-[6rem_1fr_8rem] items-center gap-x-4">
					<!-- Type label -->
					<span class="font-mono text-xs font-semibold">{prefix}</span>

					<!-- Display mode toggle -->
					<div class="flex gap-1.5">
						<button
							class="rounded border px-2 py-0.5 text-xs transition-colors {mode === 'id'
								? 'border-primary bg-primary text-primary-foreground'
								: 'border-border bg-background text-muted-foreground hover:bg-accent/50'}"
							onclick={() => handleDisplayModeChange(prefix, "id")}
						>
							ID
						</button>
						<button
							class="rounded border px-2 py-0.5 text-xs transition-colors {mode === 'title'
								? 'border-primary bg-primary text-primary-foreground'
								: 'border-border bg-background text-muted-foreground hover:bg-accent/50'}"
							onclick={() => handleDisplayModeChange(prefix, "title")}
						>
							Title
						</button>
					</div>

					<!-- Colour swatch + native picker + reset -->
					<div class="flex items-center gap-1.5">
						<label class="flex cursor-pointer items-center gap-1" title="Pick colour for {prefix}">
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
								class="text-[10px] text-muted-foreground hover:text-foreground"
								title="Reset to default"
								onclick={() => resetColor(prefix)}
							>
								↩
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</Card.Content>
</Card.Root>
