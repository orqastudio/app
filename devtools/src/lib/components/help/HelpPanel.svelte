<!-- Slide-out help panel for OrqaDev. Appears from the right side of the screen
     when the user presses ? or clicks the help icon in the toolbar. Contains three
     reference sections: keyboard shortcuts, event schema, and filter syntax.
     Closed by pressing Escape or clicking the backdrop. -->
<script lang="ts">
	import { Button, Badge, Separator, ScrollArea, CardRoot, CardContent, Kbd, Stack, HStack, Heading, Text, Caption, Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from "@orqastudio/svelte-components/pure";

	// Whether the panel is visible. Exported so the parent (DevToolsShell) can
	// toggle it from the toolbar button and the ? keyboard shortcut handler.
	let {
		open = $bindable(false),
	}: {
		open?: boolean;
	} = $props();

	// Close the panel. Called by Escape key and backdrop click.
	function close(): void {
		open = false;
	}

	// Handle keyboard events on the document: Escape closes the panel.
	function handleKeydown(e: KeyboardEvent): void {
		if (e.key === "Escape" && open) {
			e.preventDefault();
			close();
		}
	}

	// Keyboard shortcuts displayed in section 1.
	const shortcuts: [string, string][] = [
		["?", "Open / close this help panel"],
		["Ctrl+F", "Focus the search input"],
		["Ctrl+K", "Clear all log events"],
		["Ctrl+L", "Toggle scroll lock"],
		["Escape", "Close help panel or clear search"],
	];

	// Event schema fields displayed in section 2.
	const schemaFields: [string, string, string][] = [
		["id", "number", "Monotonic sequence number"],
		["timestamp", "number", "Unix time in milliseconds"],
		["level", "enum", "Debug | Info | Warn | Error | Perf"],
		["source", "enum", "Daemon | App | Frontend | DevController | MCP | LSP | Search | Worker"],
		["category", "string", "Subsystem or module name (e.g. artifact, graph)"],
		["message", "string", "Human-readable log line"],
		["metadata", "object | null", "Structured payload; schema varies by event"],
		["session_id", "string | null", "Active session when event was emitted"],
	];

	// Log level badges with semantic color classes for section 3.
	// Matches the badge variants used in LogRow and LogFilters.
	const levels: [string, string][] = [
		["Debug", "text-muted-foreground"],
		["Info", "text-primary"],
		["Warn", "text-warning"],
		["Error", "text-destructive"],
		["Perf", "text-secondary-foreground"],
	];
</script>

<svelte:document onkeydown={handleKeydown} />

{#if open}
	<!-- Backdrop: full-screen transparent overlay that closes the panel on click. -->
	<div
		class="fixed inset-0 z-40"
		role="presentation"
		onclick={close}
		aria-hidden="true"
	></div>

	<!-- Slide-out panel: fixed to the right edge, full viewport height. The outer
	     shell uses raw layout classes because component library components do not
	     model fixed-position side panels. -->
	<aside
		class="fixed right-0 top-0 z-50 flex h-full w-80 flex-col overflow-hidden border-l border-border bg-surface-base shadow-xl"
		aria-label="Help panel"
	>
		<!-- Panel header: title + ghost close button. -->
		<div class="flex h-9 shrink-0 items-center justify-between border-b border-border px-3">
			<span class="text-sm font-medium text-content-base">Help</span>
			<Button
				variant="ghost"
				size="icon-sm"
				onclick={close}
				aria-label="Close help panel"
				class="size-6 text-content-muted hover:text-content-base"
			>
				<svg width="12" height="12" viewBox="0 0 12 12" fill="none" aria-hidden="true">
					<path d="M1 1l10 10M11 1L1 11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
				</svg>
			</Button>
		</div>

		<!-- Scrollable content area via ScrollArea to get styled scrollbars. -->
		<ScrollArea class="flex-1">
			<Stack gap={0} class="px-3 py-3">

				<!-- SECTION 1: Keyboard shortcuts. Each shortcut key rendered with Kbd
				     component to communicate keyboard semantics. -->
				<CardRoot class="border-border bg-transparent shadow-none">
					<CardContent class="px-0 pb-0 pt-0">
						<Heading level={6} class="mb-2">Keyboard Shortcuts</Heading>
						<Table class="help-panel__table">
							<TableBody>
								{#each shortcuts as [key, description]}
									<TableRow class="help-panel__table-row">
										<TableCell class="help-panel__table-cell help-panel__table-cell--key">
											<Kbd>{key}</Kbd>
										</TableCell>
										<TableCell class="help-panel__table-cell">
											<Caption>{description}</Caption>
										</TableCell>
									</TableRow>
								{/each}
							</TableBody>
						</Table>
					</CardContent>
				</CardRoot>

				<Separator class="my-4" />

				<!-- SECTION 2: Event schema. Field names and types use outline Badges
				     to visually distinguish them from prose description text. -->
				<CardRoot class="border-border bg-transparent shadow-none">
					<CardContent class="px-0 pb-0 pt-0">
						<Heading level={6} class="mb-2">Event Schema</Heading>
						<Caption class="mb-2 block">Each log event has the following fields:</Caption>
						<Table class="help-panel__table">
							<TableHeader>
								<TableRow class="help-panel__table-row">
									<TableHead class="help-panel__table-head"><Caption>Field</Caption></TableHead>
									<TableHead class="help-panel__table-head"><Caption>Type</Caption></TableHead>
									<TableHead class="help-panel__table-head"><Caption>Description</Caption></TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{#each schemaFields as [field, type, desc]}
									<TableRow class="help-panel__table-row">
										<TableCell class="help-panel__table-cell">
											<Badge
												variant="outline"
												class="font-mono text-[11px] help-panel__field-badge whitespace-nowrap"
											>{field}</Badge>
										</TableCell>
										<TableCell class="help-panel__table-cell">
											<Badge
												variant="outline"
												class="font-mono text-[10px] help-panel__type-badge whitespace-nowrap"
											>{type}</Badge>
										</TableCell>
										<TableCell class="help-panel__table-cell">
											<Caption>{desc}</Caption>
										</TableCell>
									</TableRow>
								{/each}
							</TableBody>
						</Table>
					</CardContent>
				</CardRoot>

				<Separator class="my-4" />

				<!-- SECTION 3: Filter syntax. Level badges rendered with per-level
				     color classes to match the colors shown in the log table. -->
				<CardRoot class="border-border bg-transparent shadow-none">
					<CardContent class="px-0 pb-0 pt-0">
						<Heading level={6} class="mb-2">Filter Syntax</Heading>

						<Caption class="mb-3 block">
							The search box matches against the <Badge variant="outline" class="font-mono text-[10px] text-foreground">message</Badge> field as a case-insensitive substring.
						</Caption>

						<Stack gap={3}>
							<Stack gap={1}>
								<Text size="xs" class="font-semibold">Source filter</Text>
								<Caption>
									Select one or more sources from the <Text size="xs" class="font-semibold">Source</Text> dropdown.
									Only events from the selected sources are shown. Clearing the selection shows all sources.
								</Caption>
								<CardRoot class="border-border shadow-none">
									<CardContent class="px-2 py-1.5">
										<Badge variant="outline" class="font-mono text-[10px]">
											Daemon, App, Frontend, DevController, MCP, LSP, Search, Worker
										</Badge>
									</CardContent>
								</CardRoot>
							</Stack>

							<Stack gap={1}>
								<Text size="xs" class="font-semibold">Level filter</Text>
								<Caption>
									Toggle individual levels using the checkboxes in the filter bar.
									Multiple levels can be active simultaneously.
								</Caption>
								<HStack gap={1} wrap={true}>
									{#each levels as [level, cls]}
										<Badge variant="outline" class="font-mono text-[10px] {cls}">{level}</Badge>
									{/each}
								</HStack>
							</Stack>

							<Stack gap={1}>
								<Text size="xs" class="font-semibold">Category filter</Text>
								<Caption>
									Select one or more categories from the <Text size="xs" class="font-semibold">Category</Text> dropdown.
									The list is populated from categories present in the current event buffer.
								</Caption>
							</Stack>

							<Stack gap={1}>
								<Text size="xs" class="font-semibold">Combining filters</Text>
								<Caption>
									All active filters are combined with AND logic: an event must match every active
									filter to be shown. Use <Text size="xs" class="font-semibold">Clear filters</Text> to reset all at once.
								</Caption>
							</Stack>
						</Stack>
					</CardContent>
				</CardRoot>

			</Stack>
		</ScrollArea>
	</aside>
{/if}

<style>
	/* Compact table layout for help sections: removes default Table padding. */
	:global(.help-panel__table) {
		width: 100%;
	}

	:global(.help-panel__table-row) {
		border-bottom-width: 1px;
	}

	:global(.help-panel__table-head) {
		padding: 4px 12px 4px 0;
		text-align: left;
		vertical-align: top;
	}

	:global(.help-panel__table-cell) {
		padding: 6px 12px 6px 0;
		vertical-align: top;
	}

	:global(.help-panel__table-cell--key) {
		white-space: nowrap;
	}

	/* Field badge: primary color for field names in the event schema. */
	:global(.help-panel__field-badge) {
		color: var(--color-primary) !important;
	}

	/* Type badge: secondary foreground for type annotations. */
	:global(.help-panel__type-badge) {
		color: var(--color-secondary-foreground) !important;
	}
</style>
