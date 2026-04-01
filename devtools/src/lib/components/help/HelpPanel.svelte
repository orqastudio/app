<!-- Slide-out help panel for OrqaDev. Appears from the right side of the screen
     when the user presses ? or clicks the help icon in the toolbar. Contains three
     hardcoded reference sections: keyboard shortcuts, event schema, and filter syntax.
     Closed by pressing Escape or clicking the backdrop. -->
<script lang="ts">
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

	<!-- Slide-out panel: fixed to the right edge, full viewport height. Uses a
	     translate animation driven by the `open` state. -->
	<aside
		class="fixed right-0 top-0 z-50 flex h-full w-80 flex-col overflow-hidden border-l border-border bg-surface-base shadow-xl"
		aria-label="Help panel"
	>
		<!-- Panel header -->
		<div class="flex h-9 shrink-0 items-center justify-between border-b border-border px-3">
			<span class="text-sm font-medium text-content-base">Help</span>
			<button
				class="flex size-6 items-center justify-center rounded text-content-muted transition-colors hover:bg-surface-raised hover:text-content-base focus:outline-none focus-visible:ring-1 focus-visible:ring-blue-500"
				onclick={close}
				aria-label="Close help panel"
			>
				<svg width="12" height="12" viewBox="0 0 12 12" fill="none" aria-hidden="true">
					<path d="M1 1l10 10M11 1L1 11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
				</svg>
			</button>
		</div>

		<!-- Scrollable content area -->
		<div class="flex-1 overflow-y-auto px-3 py-3 text-[12px]">

			<!-- SECTION 1: Keyboard shortcuts -->
			<section class="mb-5">
				<h2 class="mb-2 text-[11px] font-semibold uppercase tracking-wider text-content-muted">
					Keyboard Shortcuts
				</h2>
				<table class="w-full border-collapse">
					<tbody>
						{#each [
							["?", "Open / close this help panel"],
							["Ctrl+F", "Focus the search input"],
							["Ctrl+K", "Clear all log events"],
							["Ctrl+L", "Toggle scroll lock"],
							["Escape", "Close help panel or clear search"],
						] as [key, description]}
							<tr class="border-b border-border/40 last:border-0">
								<td class="py-1.5 pr-3 align-top">
									<kbd class="inline-block rounded border border-border bg-surface-raised px-1.5 py-0.5 font-mono text-[10px] text-content-base leading-none whitespace-nowrap">
										{key}
									</kbd>
								</td>
								<td class="py-1.5 text-content-base/80 align-top">{description}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</section>

			<!-- SECTION 2: Event schema -->
			<section class="mb-5">
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
						{#each [
							["id", "number", "Monotonic sequence number"],
							["timestamp", "number", "Unix time in milliseconds"],
							["level", "enum", "Debug | Info | Warn | Error | Perf"],
							["source", "enum", "Daemon | App | Frontend | DevController | MCP | LSP | Search | Worker"],
							["category", "string", "Subsystem or module name (e.g. artifact, graph)"],
							["message", "string", "Human-readable log line"],
							["metadata", "object | null", "Structured payload; schema varies by event"],
							["session_id", "string | null", "Active session when event was emitted"],
						] as [field, type, desc]}
							<tr class="border-b border-border/40 last:border-0">
								<td class="py-1.5 pr-3 align-top font-mono text-[11px] text-blue-400 whitespace-nowrap">{field}</td>
								<td class="py-1.5 pr-3 align-top font-mono text-[10px] text-indigo-400 whitespace-nowrap">{type}</td>
								<td class="py-1.5 text-content-base/80 align-top">{desc}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</section>

			<!-- SECTION 3: Filter syntax -->
			<section class="mb-3">
				<h2 class="mb-2 text-[11px] font-semibold uppercase tracking-wider text-content-muted">
					Filter Syntax
				</h2>

				<p class="mb-2 text-content-muted">
					The search box matches against the <span class="font-mono text-content-base">message</span> field as a case-insensitive substring.
				</p>

				<div class="mb-3">
					<p class="mb-1 font-semibold text-content-base">Source filter</p>
					<p class="text-content-muted">
						Select one or more sources from the <span class="font-semibold text-content-base">Source</span> dropdown.
						Only events from the selected sources are shown. Clearing the selection shows all sources.
					</p>
					<div class="mt-1.5 rounded border border-border bg-surface-raised px-2 py-1.5 font-mono text-[11px] text-content-base">
						Daemon, App, Frontend, DevController, MCP, LSP, Search, Worker
					</div>
				</div>

				<div class="mb-3">
					<p class="mb-1 font-semibold text-content-base">Level filter</p>
					<p class="text-content-muted">
						Toggle individual levels using the checkboxes in the filter bar.
						Multiple levels can be active simultaneously.
					</p>
					<div class="mt-1.5 flex flex-wrap gap-1">
						{#each [
							["Debug", "text-content-muted"],
							["Info", "text-blue-400"],
							["Warn", "text-yellow-400"],
							["Error", "text-red-400"],
							["Perf", "text-indigo-400"],
						] as [level, cls]}
							<span class="rounded border border-border bg-surface-raised px-1.5 py-0.5 font-mono text-[10px] {cls}">{level}</span>
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
			</section>
		</div>
	</aside>
{/if}
