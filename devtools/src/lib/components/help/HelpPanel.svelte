<!-- Slide-out help panel for OrqaDev. Appears from the right side of the screen
     when the user presses ? or clicks the help icon in the toolbar. Contains three
     reference sections: keyboard shortcuts, event schema, and filter syntax.
     Closed by pressing Escape or clicking the backdrop. -->
<script lang="ts">
	import {
		Button,
		Badge,
		Panel,
		SectionHeader,
		Separator,
		ScrollArea,
		CardRoot,
		CardContent,
		Kbd,
		Stack,
		HStack,
		Heading,
		Text,
		Caption,
		Table,
		TableHeader,
		TableBody,
		TableRow,
		TableHead,
		TableCell,
		Icon,
		Box,
		SidePanel,
	} from "@orqastudio/svelte-components/pure";

	// Whether the panel is visible. Exported so the parent (DevToolsShell) can
	// toggle it from the toolbar button and the ? keyboard shortcut handler.
	let {
		open = $bindable(false),
	}: {
		open?: boolean;
	} = $props();

	/** Close the help panel. Called by Escape key and backdrop click. */
	function close(): void {
		open = false;
	}

	/**
	 * Handle document keydown: Escape closes the panel when open.
	 * @param e - The keyboard event from the document keydown listener.
	 */
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

	// Badge variant for each log level — maps to the same variants used in LogRow and LogFilters.
	const LEVEL_BADGE_VARIANT: Record<
		string,
		"secondary" | "destructive" | "outline" | "default" | "warning"
	> = {
		Debug: "outline",
		Info: "default",
		Warn: "warning",
		Error: "destructive",
		Perf: "secondary",
	};

	const ALL_LEVELS = ["Debug", "Info", "Warn", "Error", "Perf"] as const;
</script>

<svelte:document onkeydown={handleKeydown} />

{#if open}
	<!-- Backdrop: full-screen transparent overlay that closes the panel on click. -->
	<Box
		position="fixed"
		inset={0}
		zIndex={40}
		role="presentation"
		aria-hidden={true}
		onclick={close}
	/>

	<!-- Slide-out panel: fixed to the right edge, full viewport height. -->
	<Box position="fixed" right={0} top={0} zIndex={50} aria-label="Help panel">
		<SidePanel role="complementary" aria-label="Help panel">
			<!-- Panel header: SectionHeader provides px-3 py-2 border-b layout. -->
			<SectionHeader>
				{#snippet start()}
					<Text variant="body-strong">Help</Text>
				{/snippet}
				{#snippet end()}
					<Button variant="ghost" size="icon-sm" onclick={close} aria-label="Close help panel">
						<Icon name="x" size="sm" />
					</Button>
				{/snippet}
			</SectionHeader>

			<!-- Scrollable content area via ScrollArea for styled scrollbars. -->
			<ScrollArea full>
				<Panel padding="normal">
					<Stack gap={0}>
						<!-- SECTION 1: Keyboard shortcuts. -->
						<CardRoot>
							<CardContent>
								<Stack gap={2}>
									<Heading level={6}>Keyboard Shortcuts</Heading>
									<Table>
										<TableBody>
											{#each shortcuts as [key, description] (key)}
												<TableRow>
													<TableCell>
														<Kbd>{key}</Kbd>
													</TableCell>
													<TableCell>
														<Caption>{description}</Caption>
													</TableCell>
												</TableRow>
											{/each}
										</TableBody>
									</Table>
								</Stack>
							</CardContent>
						</CardRoot>

						<Separator />

						<!-- SECTION 2: Event schema. -->
						<CardRoot>
							<CardContent>
								<Stack gap={2}>
									<Heading level={6}>Event Schema</Heading>
									<Caption block>Each log event has the following fields:</Caption>
									<Table>
										<TableHeader>
											<TableRow>
												<TableHead><Caption>Field</Caption></TableHead>
												<TableHead><Caption>Type</Caption></TableHead>
												<TableHead><Caption>Description</Caption></TableHead>
											</TableRow>
										</TableHeader>
										<TableBody>
											{#each schemaFields as [field, type, desc] (field)}
												<TableRow>
													<TableCell>
														<Badge variant="outline" size="xs">{field}</Badge>
													</TableCell>
													<TableCell>
														<Badge variant="outline" size="xs">{type}</Badge>
													</TableCell>
													<TableCell>
														<Caption>{desc}</Caption>
													</TableCell>
												</TableRow>
											{/each}
										</TableBody>
									</Table>
								</Stack>
							</CardContent>
						</CardRoot>

						<Separator />

						<!-- SECTION 3: Filter syntax. -->
						<CardRoot>
							<CardContent>
								<Stack gap={3}>
									<Heading level={6}>Filter Syntax</Heading>

									<Caption block>
										The search box matches against the <Badge variant="outline" size="xs"
											>message</Badge
										> field as a case-insensitive substring.
									</Caption>

									<Stack gap={1}>
										<Text variant="body-strong">Source filter</Text>
										<Caption block>
											Select one or more sources from the <Text variant="body-strong">Source</Text> dropdown.
											Only events from the selected sources are shown. Clearing the selection shows all
											sources.
										</Caption>
										<CardRoot>
											<CardContent>
												<Badge variant="outline" size="xs">
													Daemon, App, Frontend, DevController, MCP, LSP, Search, Worker
												</Badge>
											</CardContent>
										</CardRoot>
									</Stack>

									<Stack gap={1}>
										<Text variant="body-strong">Level filter</Text>
										<Caption block>
											Toggle individual levels using the checkboxes in the filter bar. Multiple
											levels can be active simultaneously.
										</Caption>
										<HStack gap={1} wrap={true}>
											{#each ALL_LEVELS as level (level)}
												<Badge variant={LEVEL_BADGE_VARIANT[level] ?? "outline"} size="xs"
													>{level}</Badge
												>
											{/each}
										</HStack>
									</Stack>

									<Stack gap={1}>
										<Text variant="body-strong">Category filter</Text>
										<Caption block>
											Select one or more categories from the <Text variant="body-strong"
												>Category</Text
											> dropdown. The list is populated from categories present in the current event
											buffer.
										</Caption>
									</Stack>

									<Stack gap={1}>
										<Text variant="body-strong">Combining filters</Text>
										<Caption block>
											All active filters are combined with AND logic: an event must match every
											active filter to be shown. Use <Text variant="body-strong">Clear filters</Text
											> to reset all at once.
										</Caption>
									</Stack>
								</Stack>
							</CardContent>
						</CardRoot>
					</Stack>
				</Panel>
			</ScrollArea>
		</SidePanel>
	</Box>
{/if}
