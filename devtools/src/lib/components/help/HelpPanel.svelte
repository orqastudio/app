<!-- Slide-out help panel for OrqaDev. Appears from the right side of the screen
     when the user presses ? or clicks the help icon in the toolbar. Contains three
     reference sections: keyboard shortcuts, event schema, and filter syntax.
     Closed by pressing Escape or clicking the backdrop. -->
<script lang="ts">
	import { Button, Badge, Separator, ScrollArea, CardRoot, CardContent } from "@orqastudio/svelte-components/pure";

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

	// Log level badges with their display colors for section 3.
	const levels: [string, string][] = [
		["Debug", "text-content-muted"],
		["Info", "text-blue-400"],
		["Warn", "text-yellow-400"],
		["Error", "text-red-400"],
		["Perf", "text-indigo-400"],
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
			<div class="px-3 py-3 text-[12px]">

				<!-- SECTION 1: Keyboard shortcuts. Each shortcut key rendered as an
				     outline Badge to communicate its <kbd> semantics visually. -->
				<CardRoot class="border-border bg-transparent shadow-none">
					<CardContent class="px-0 pb-0 pt-0">
						<h2 class="mb-2 text-[11px] font-semibold uppercase tracking-wider text-content-muted">
							Keyboard Shortcuts
						</h2>
						<table class="w-full border-collapse">
							<tbody>
								{#each shortcuts as [key, description]}
									<tr class="border-b border-border/40 last:border-0">
										<td class="py-1.5 pr-3 align-top">
											<Badge
												variant="outline"
												class="font-mono text-[10px] leading-none whitespace-nowrap text-content-base"
											>{key}</Badge>
										</td>
										<td class="py-1.5 text-content-base/80 align-top">{description}</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</CardContent>
				</CardRoot>

				<Separator class="my-4" />

				<!-- SECTION 2: Event schema. Field names and types use outline Badges
				     to visually distinguish them from prose description text. -->
				<CardRoot class="border-border bg-transparent shadow-none">
					<CardContent class="px-0 pb-0 pt-0">
						<h2 class="mb-2 text-[11px] font-semibold uppercase tracking-wider text-content-muted">
							Event Schema
						</h2>
						<p class="mb-2 text-content-muted">
							Each log event has the following fields:
						</p>
						<table class="w-full border-collapse">
							<thead>
								<tr class="border-b border-border">
									<th class="pb-1 pr-3 text-left text-[10px] font-semibold text-content-muted">Field</th>
									<th class="pb-1 pr-3 text-left text-[10px] font-semibold text-content-muted">Type</th>
									<th class="pb-1 text-left text-[10px] font-semibold text-content-muted">Description</th>
								</tr>
							</thead>
							<tbody>
								{#each schemaFields as [field, type, desc]}
									<tr class="border-b border-border/40 last:border-0">
										<td class="py-1.5 pr-3 align-top">
											<Badge
												variant="outline"
												class="font-mono text-[11px] text-blue-400 whitespace-nowrap"
											>{field}</Badge>
										</td>
										<td class="py-1.5 pr-3 align-top">
											<Badge
												variant="outline"
												class="font-mono text-[10px] text-indigo-400 whitespace-nowrap"
											>{type}</Badge>
										</td>
										<td class="py-1.5 text-content-base/80 align-top">{desc}</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</CardContent>
				</CardRoot>

				<Separator class="my-4" />

				<!-- SECTION 3: Filter syntax. Level badges rendered with per-level
				     color classes to match the colors shown in the log table. -->
				<CardRoot class="border-border bg-transparent shadow-none">
					<CardContent class="px-0 pb-0 pt-0">
						<h2 class="mb-2 text-[11px] font-semibold uppercase tracking-wider text-content-muted">
							Filter Syntax
						</h2>

						<p class="mb-3 text-content-muted">
							The search box matches against the <Badge variant="outline" class="font-mono text-[10px] text-content-base">message</Badge> field as a case-insensitive substring.
						</p>

						<div class="mb-3">
							<p class="mb-1 font-semibold text-content-base">Source filter</p>
							<p class="mb-1.5 text-content-muted">
								Select one or more sources from the <span class="font-semibold text-content-base">Source</span> dropdown.
								Only events from the selected sources are shown. Clearing the selection shows all sources.
							</p>
							<CardRoot class="border-border shadow-none">
								<CardContent class="px-2 py-1.5">
									<span class="font-mono text-[11px] text-content-base">
										Daemon, App, Frontend, DevController, MCP, LSP, Search, Worker
									</span>
								</CardContent>
							</CardRoot>
						</div>

						<div class="mb-3">
							<p class="mb-1 font-semibold text-content-base">Level filter</p>
							<p class="mb-1.5 text-content-muted">
								Toggle individual levels using the checkboxes in the filter bar.
								Multiple levels can be active simultaneously.
							</p>
							<div class="flex flex-wrap gap-1">
								{#each levels as [level, cls]}
									<Badge variant="outline" class="font-mono text-[10px] {cls}">{level}</Badge>
								{/each}
							</div>
						</div>

						<div class="mb-3">
							<p class="mb-1 font-semibold text-content-base">Category filter</p>
							<p class="text-content-muted">
								Select one or more categories from the <span class="font-semibold text-content-base">Category</span> dropdown.
								The list is populated from categories present in the current event buffer.
							</p>
						</div>

						<div>
							<p class="mb-1 font-semibold text-content-base">Combining filters</p>
							<p class="text-content-muted">
								All active filters are combined with AND logic: an event must match every active
								filter to be shown. Use <span class="font-semibold text-content-base">Clear filters</span> to reset all at once.
							</p>
						</div>
					</CardContent>
				</CardRoot>

			</div>
		</ScrollArea>
	</aside>
{/if}
