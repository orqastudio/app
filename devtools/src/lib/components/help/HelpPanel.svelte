<!-- Slide-out help panel for OrqaDev. Appears from the right side of the screen
     when the user presses ? or clicks the help icon in the toolbar. Contains three
     reference sections: keyboard shortcuts, event schema, and filter syntax.
     Closed by pressing Escape or clicking the backdrop. -->
<script lang="ts">
	import { Button, Badge, Separator, ScrollArea, CardRoot, CardContent, Kbd, Stack, HStack, Heading, Text, Caption, Table, TableHeader, TableBody, TableRow, TableHead, TableCell, Icon, Box } from "@orqastudio/svelte-components/pure";

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

	// Log level tone classes for section 3: maps level names to Text tone values.
	// Matches the badge variants used in LogRow and LogFilters.
	const levels: [string, "muted" | "warning" | "destructive" | "success" | undefined][] = [
		["Debug", "muted"],
		["Info", undefined],
		["Warn", "warning"],
		["Error", "destructive"],
		["Perf", undefined],
	];
</script>

<svelte:document onkeydown={handleKeydown} />

{#if open}
	<!-- Backdrop: full-screen transparent overlay that closes the panel on click.
	     Raw div used because Box does not accept aria-hidden. -->
	<div
		class="help-panel__backdrop"
		role="presentation"
		onclick={close}
		aria-hidden="true"
	></div>

	<!-- Slide-out panel: fixed to the right edge, full viewport height. -->
	<Box
		position="fixed"
		right={0}
		top={0}
		zIndex={50}
		overflow="hidden"
		aria-label="Help panel"
	>
		<!-- Panel wrapper: full height flex column, scoped CSS provides width and border. -->
		<div class="help-panel" role="complementary">
			<!-- Panel header: HStack provides flex-row layout. Scoped CSS provides height/border. -->
			<div class="help-panel__header">
				<HStack justify="between" full paddingX={3}>
					<!-- Text body-strong replaces raw span with font-medium. -->
					<Text variant="body-strong">Help</Text>
					<!-- Wrapper span with display:contents provides :global() hook for Button override. -->
					<span class="help-panel__close-wrap" style="display: contents;">
						<Button
							variant="ghost"
							size="icon-sm"
							onclick={close}
							aria-label="Close help panel"
						>
							<!-- Icon component replaces the raw inline SVG. Size "sm" = 14px. -->
							<Icon name="x" size="sm" />
						</Button>
					</span>
				</HStack>
			</div>

			<!-- Scrollable content area via ScrollArea for styled scrollbars. -->
			<ScrollArea full>
				<Stack gap={0} paddingX={3} paddingY={3}>

					<!-- SECTION 1: Keyboard shortcuts. -->
					<CardRoot>
						<CardContent>
							<div class="help-panel__section-head"><Heading level={6}>Keyboard Shortcuts</Heading></div>
							<!-- Wrapper div provides :global() scope for table cell overrides. -->
							<div class="help-panel__table-wrap">
								<Table>
									<TableBody>
										{#each shortcuts as [key, description]}
											<TableRow>
												<TableCell>
													<span class="help-panel__key-cell">
														<Kbd>{key}</Kbd>
													</span>
												</TableCell>
												<TableCell>
													<Caption>{description}</Caption>
												</TableCell>
											</TableRow>
										{/each}
									</TableBody>
								</Table>
							</div>
						</CardContent>
					</CardRoot>

					<Separator />

					<!-- SECTION 2: Event schema. -->
					<CardRoot>
						<CardContent>
							<div class="help-panel__section-head"><Heading level={6}>Event Schema</Heading></div>
							<div class="help-panel__section-sub"><Caption block>Each log event has the following fields:</Caption></div>
							<div class="help-panel__table-wrap">
								<Table>
									<TableHeader>
										<TableRow>
											<TableHead><Caption>Field</Caption></TableHead>
											<TableHead><Caption>Type</Caption></TableHead>
											<TableHead><Caption>Description</Caption></TableHead>
										</TableRow>
									</TableHeader>
									<TableBody>
										{#each schemaFields as [field, type, desc]}
											<TableRow>
												<TableCell>
													<!-- Wrapper span for field badge monospace/primary color override. -->
													<span class="help-panel__field-badge-wrap" style="display: contents;">
														<Badge variant="outline" size="xs">{field}</Badge>
													</span>
												</TableCell>
												<TableCell>
													<!-- Wrapper span for type badge monospace/secondary color override. -->
													<span class="help-panel__type-badge-wrap" style="display: contents;">
														<Badge variant="outline" size="xs">{type}</Badge>
													</span>
												</TableCell>
												<TableCell>
													<Caption>{desc}</Caption>
												</TableCell>
											</TableRow>
										{/each}
									</TableBody>
								</Table>
							</div>
						</CardContent>
					</CardRoot>

					<Separator />

					<!-- SECTION 3: Filter syntax. -->
					<CardRoot>
						<CardContent>
							<div class="help-panel__section-head"><Heading level={6}>Filter Syntax</Heading></div>

							<div class="help-panel__section-sub">
								<Caption block>
									The search box matches against the <Badge variant="outline" size="xs">message</Badge> field as a case-insensitive substring.
								</Caption>
							</div>

							<Stack gap={3}>
								<Stack gap={1}>
									<Text variant="body-strong">Source filter</Text>
									<Caption block>
										Select one or more sources from the <Text variant="body-strong">Source</Text> dropdown.
										Only events from the selected sources are shown. Clearing the selection shows all sources.
									</Caption>
									<CardRoot>
										<CardContent>
											<!-- Wrapper span for mono badge override. -->
											<span class="help-panel__mono-badge-wrap" style="display: contents;">
												<Badge variant="outline" size="xs">
													Daemon, App, Frontend, DevController, MCP, LSP, Search, Worker
												</Badge>
											</span>
										</CardContent>
									</CardRoot>
								</Stack>

								<Stack gap={1}>
									<Text variant="body-strong">Level filter</Text>
									<Caption block>
										Toggle individual levels using the checkboxes in the filter bar.
										Multiple levels can be active simultaneously.
									</Caption>
									<HStack gap={1} wrap={true}>
										{#each levels as [level, tone]}
											<!-- Badge provides the visual level pill; data-level drives color
											     via scoped CSS. Now accepted via Badge restProps. -->
											<Badge variant="outline" size="xs" data-level={level}>{level}</Badge>
										{/each}
									</HStack>
								</Stack>

								<Stack gap={1}>
									<Text variant="body-strong">Category filter</Text>
									<Caption block>
										Select one or more categories from the <Text variant="body-strong">Category</Text> dropdown.
										The list is populated from categories present in the current event buffer.
									</Caption>
								</Stack>

								<Stack gap={1}>
									<Text variant="body-strong">Combining filters</Text>
									<Caption block>
										All active filters are combined with AND logic: an event must match every active
										filter to be shown. Use <Text variant="body-strong">Clear filters</Text> to reset all at once.
									</Caption>
								</Stack>
							</Stack>
						</CardContent>
					</CardRoot>

				</Stack>
			</ScrollArea>
		</div>
	</Box>
{/if}

<style>
	/* Backdrop: full-screen fixed overlay, invisible to layout. */
	.help-panel__backdrop {
		position: fixed;
		inset: 0;
		z-index: 40;
	}

	/* Side panel: fixed width, full height, slide-in from right. */
	.help-panel {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 20rem; /* w-80 */
		border-left: 1px solid var(--color-border);
		background-color: var(--color-surface-base);
		box-shadow: var(--shadow-xl);
	}

	/* Panel header: compact fixed-height bar with bottom border. */
	.help-panel__header {
		display: flex;
		align-items: center;
		height: 2.25rem; /* h-9 */
		flex-shrink: 0;
		border-bottom: 1px solid var(--color-border);
	}

	/* Close button: compact, muted color.
	   Targets Button inside the close wrapper span. */
	:global(.help-panel__close-wrap button) {
		width: 1.5rem !important;
		height: 1.5rem !important;
		color: var(--color-content-muted) !important;
	}

	:global(.help-panel__close-wrap button:hover) {
		color: var(--color-content-base) !important;
	}

	/* Section heading: bottom margin via scoped div. */
	.help-panel__section-head {
		margin-bottom: var(--spacing-2);
	}

	/* Section sub-text: bottom margin via scoped div. */
	.help-panel__section-sub {
		margin-bottom: var(--spacing-2);
	}

	/* Table wrapper: provides scoped descendant selector context for cell overrides. */
	.help-panel__table-wrap {
		width: 100%;
	}

	/* Compact table layout for help sections. */
	:global(.help-panel__table-wrap [data-slot="table"]) {
		width: 100%;
	}

	:global(.help-panel__table-wrap [data-slot="table-row"]) {
		border-bottom-width: 1px;
	}

	:global(.help-panel__table-wrap [data-slot="table-head"]) {
		padding: 4px 12px 4px 0;
		text-align: left;
		vertical-align: top;
	}

	:global(.help-panel__table-wrap [data-slot="table-cell"]) {
		padding: 6px 12px 6px 0;
		vertical-align: top;
	}

	/* Key cell: no-wrap for the Kbd shortcut column. */
	.help-panel__key-cell {
		white-space: nowrap;
	}

	/* Field badge: primary color for field names in the event schema.
	   Targets Badge inside the field badge wrapper span. */
	:global(.help-panel__field-badge-wrap [data-slot="badge"]) {
		font-family: var(--font-mono) !important;
		color: var(--color-primary) !important;
		white-space: nowrap;
	}

	/* Type badge: secondary foreground for type annotations.
	   Targets Badge inside the type badge wrapper span. */
	:global(.help-panel__type-badge-wrap [data-slot="badge"]) {
		font-family: var(--font-mono) !important;
		color: var(--color-secondary-foreground) !important;
		white-space: nowrap;
	}

	/* Mono badge for code examples.
	   Targets Badge inside the mono badge wrapper span. */
	:global(.help-panel__mono-badge-wrap [data-slot="badge"]) {
		font-family: var(--font-mono) !important;
	}

	/* Level badges: data-level drives color to match LogRow/LogFilters.
	   Targets Badge elements with data-level attribute inside the panel. */
	:global(.help-panel [data-slot="badge"][data-level]) {
		font-family: var(--font-mono) !important;
	}

	:global(.help-panel [data-slot="badge"][data-level="Debug"]) {
		color: var(--color-content-muted) !important;
	}

	:global(.help-panel [data-slot="badge"][data-level="Info"]) {
		color: var(--color-primary) !important;
	}

	:global(.help-panel [data-slot="badge"][data-level="Warn"]) {
		color: var(--color-warning) !important;
	}

	:global(.help-panel [data-slot="badge"][data-level="Error"]) {
		color: var(--color-destructive) !important;
	}

	:global(.help-panel [data-slot="badge"][data-level="Perf"]) {
		color: var(--color-secondary-foreground) !important;
	}
</style>
