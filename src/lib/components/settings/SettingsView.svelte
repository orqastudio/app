<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import * as Card from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge";
	import { Separator } from "$lib/components/ui/separator";
	import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
	import CircleXIcon from "@lucide/svelte/icons/circle-x";
	import { projectStore } from "$lib/stores/project.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";

	const project = $derived(projectStore.activeProject);

	const shortcuts: { key: string; action: string }[] = [
		{ key: "Ctrl+K", action: "Global search" },
		{ key: "Ctrl+N", action: "New session" },
		{ key: "Ctrl+B", action: "Toggle Nav Sub-Panel" },
		{ key: "Ctrl+,", action: "Open settings" },
		{ key: "Ctrl+0", action: "Project Dashboard" },
		{ key: "Ctrl+1-5", action: "Switch artifact category" },
		{ key: "Ctrl+E", action: "Toggle edit mode" },
		{ key: "Ctrl+S", action: "Save (in edit mode)" },
		{ key: "Escape", action: "Close overlay / cancel" },
	];

	const themeModeOptions: { value: "light" | "dark" | "system"; label: string }[] = [
		{ value: "system", label: "System (default)" },
		{ value: "light", label: "Light" },
		{ value: "dark", label: "Dark" },
	];

	const fontSizeOptions = [12, 13, 14, 15, 16, 18, 20];
</script>

<ScrollArea.Root class="h-full">
	<div class="space-y-6 p-6">
		<!-- Provider section -->
		{#if settingsStore.activeSection === "provider"}
			<Card.Root>
				<Card.Header>
					<Card.Title>Provider</Card.Title>
					<Card.Description>Claude Code CLI connection and sidecar status</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					<div>
						<label class="text-sm font-medium" for="cli-path">Claude Code CLI Path</label>
						<div class="mt-1 flex gap-2">
							<input
								id="cli-path"
								class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm"
								value="Not configured"
								disabled
							/>
							<button class="rounded-md border border-border px-3 py-1.5 text-sm hover:bg-accent/50">
								Browse
							</button>
							<button class="rounded-md border border-border px-3 py-1.5 text-sm hover:bg-accent/50">
								Auto-detect
							</button>
						</div>
					</div>

					<Separator />

					<div class="space-y-2">
						<div class="flex items-center gap-2 text-sm">
							<span class="text-muted-foreground">Sidecar Status:</span>
							<div class="flex items-center gap-1">
								<CircleXIcon class="h-4 w-4 text-muted-foreground" />
								<span class="text-muted-foreground">Not started</span>
							</div>
						</div>
						<div class="flex items-center gap-2 text-sm">
							<span class="text-muted-foreground">Connection Health:</span>
							<span class="text-muted-foreground">Not connected</span>
						</div>
					</div>

					<button class="rounded-md border border-border px-3 py-1.5 text-sm hover:bg-accent/50">
						Restart Sidecar
					</button>
				</Card.Content>
			</Card.Root>
		{/if}

		<!-- Project section -->
		{#if settingsStore.activeSection === "project"}
			<Card.Root>
				<Card.Header>
					<Card.Title>Project</Card.Title>
					<Card.Description>Active project information and scan settings</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					{#if project}
						<div>
							<label class="text-sm font-medium" for="project-root">Project Root</label>
							<div class="mt-1 flex gap-2">
								<input
									id="project-root"
									class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm"
									value={project.path}
									disabled
								/>
								<button class="rounded-md border border-border px-3 py-1.5 text-sm hover:bg-accent/50">
									Change
								</button>
							</div>
						</div>

						{#if project.detected_stack}
							<Separator />
							<div class="space-y-1 text-sm">
								<h4 class="font-medium">Detected Stack</h4>
								<p class="text-muted-foreground">
									Languages: {project.detected_stack.languages.join(", ") || "None"}
								</p>
								<p class="text-muted-foreground">
									Frameworks: {project.detected_stack.frameworks.join(", ") || "None"}
								</p>
							</div>
						{/if}

						<button class="rounded-md border border-border px-3 py-1.5 text-sm hover:bg-accent/50">
							Rescan Project
						</button>
					{:else}
						<div class="flex items-center gap-2 text-sm text-muted-foreground">
							<CircleXIcon class="h-4 w-4" />
							No project loaded
						</div>
					{/if}
				</Card.Content>
			</Card.Root>
		{/if}

		<!-- Appearance section -->
		{#if settingsStore.activeSection === "appearance"}
			<Card.Root>
				<Card.Header>
					<Card.Title>Appearance</Card.Title>
					<Card.Description>Theme, font size, and display preferences</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					<div>
						<label class="text-sm font-medium" for="theme-select">Theme</label>
						<select
							id="theme-select"
							class="mt-1 flex h-9 w-full max-w-xs rounded-md border border-input bg-background px-3 py-1 text-sm"
							value={settingsStore.themeMode}
							onchange={(e: Event) => {
								const target = e.target as HTMLSelectElement;
								settingsStore.setThemeMode(target.value as "light" | "dark" | "system");
							}}
						>
							{#each themeModeOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>

					<div>
						<label class="text-sm font-medium" for="font-size-select">Font Size</label>
						<select
							id="font-size-select"
							class="mt-1 flex h-9 w-full max-w-xs rounded-md border border-input bg-background px-3 py-1 text-sm"
							value={settingsStore.fontSize}
							onchange={(e: Event) => {
								const target = e.target as HTMLSelectElement;
								settingsStore.setFontSize(parseInt(target.value, 10));
							}}
						>
							{#each fontSizeOptions as size}
								<option value={size}>{size}px</option>
							{/each}
						</select>
					</div>
				</Card.Content>
			</Card.Root>
		{/if}

		<!-- Keyboard shortcuts section -->
		{#if settingsStore.activeSection === "shortcuts"}
			<Card.Root>
				<Card.Header>
					<Card.Title>Keyboard Shortcuts</Card.Title>
					<Card.Description>Reference card for available shortcuts</Card.Description>
				</Card.Header>
				<Card.Content>
					<div class="space-y-2">
						{#each shortcuts as shortcut}
							<div class="flex items-center justify-between rounded px-2 py-1.5 text-sm hover:bg-muted/50">
								<span class="text-muted-foreground">{shortcut.action}</span>
								<Badge variant="outline" class="font-mono text-xs">{shortcut.key}</Badge>
							</div>
						{/each}
					</div>
				</Card.Content>
			</Card.Root>
		{/if}
	</div>
</ScrollArea.Root>
