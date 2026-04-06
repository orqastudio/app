<script lang="ts">
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { Button, HStack, Stack, Caption, Panel } from "@orqastudio/svelte-components/pure";
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
		<Panel padding="tight">
		<HStack gap={4}>
			<Caption variant="caption-strong" tone="muted">Type</Caption>
			<Caption variant="caption-strong" tone="muted">Display</Caption>
			<Caption variant="caption-strong" tone="muted">Colour</Caption>
		</HStack>
		</Panel>

		<Separator />

		<!-- Per-type rows -->
		<Stack gap={1}>
			{#each prefixes as prefix (prefix)}
				{@const color = effectiveColors[prefix] ?? "#64748b"}
				{@const isDefault = false}
				{@const mode = getDisplayMode(prefix)}

				<HStack gap={4}>
					<!-- Type label -->
					<Caption variant="caption-mono">{prefix}</Caption>

					<!-- Display mode toggle -->
					<HStack gap={1}>
						<Button
							variant={mode === "id" ? "default" : "outline"}
							size="sm"
							onclick={() => handleDisplayModeChange(prefix, "id")}
						>
							ID
						</Button>
						<Button
							variant={mode === "title" ? "default" : "outline"}
							size="sm"
							onclick={() => handleDisplayModeChange(prefix, "title")}
						>
							Title
						</Button>
					</HStack>

					<!-- Colour swatch + native picker + reset.
					     input[type=color] is a legitimate exception — ColorInput primitive follow-up needed. -->
					<HStack gap={1}>
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
							<Button
								variant="ghost"
								size="sm"
								aria-label="Reset to default"
								onclick={() => resetColor(prefix)}
							>
								↩
							</Button>
						{/if}
					</HStack>
				</HStack>
			{/each}
		</Stack>
	</CardContent>
</CardRoot>
