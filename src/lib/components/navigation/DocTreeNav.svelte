<script lang="ts">
	import * as Collapsible from "$lib/components/ui/collapsible";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import { navigationStore } from "$lib/stores/navigation.svelte";

	interface DocNode {
		label: string;
		path?: string;
		children?: DocNode[];
	}

	const docTree: DocNode[] = [
		{
			label: "Product",
			children: [
				{ label: "Vision", path: "product/vision" },
				{ label: "Governance", path: "product/governance" },
				{ label: "Personas", path: "product/personas" },
				{ label: "Journeys", path: "product/journeys" },
				{ label: "Information Architecture", path: "product/information-architecture" },
				{ label: "MVP Specification", path: "product/mvp-specification" },
				{ label: "Glossary", path: "product/glossary" },
				{ label: "Roadmap", path: "product/roadmap" },
			],
		},
		{
			label: "Architecture",
			children: [
				{ label: "Decisions", path: "architecture/decisions" },
				{ label: "IPC Commands", path: "architecture/ipc-commands" },
				{ label: "Rust Modules", path: "architecture/rust-modules" },
				{ label: "Svelte Components", path: "architecture/svelte-components" },
				{ label: "Streaming Pipeline", path: "architecture/streaming-pipeline" },
				{ label: "Tool Definitions", path: "architecture/tool-definitions" },
				{ label: "MCP Host", path: "architecture/mcp-host" },
				{ label: "Error Taxonomy", path: "architecture/error-taxonomy" },
				{ label: "SQLite Schema", path: "architecture/sqlite-schema" },
				{ label: "Wireframe Serving", path: "architecture/wireframe-serving" },
			],
		},
		{
			label: "UI",
			children: [
				{ label: "Design System", path: "ui/design-system" },
				{ label: "Brand Identity", path: "ui/brand-identity" },
				{ label: "Component Inventory", path: "ui/component-inventory" },
				{ label: "Interaction Patterns", path: "ui/interaction-patterns" },
				{ label: "Responsive Behavior", path: "ui/responsive-behavior" },
				{
					label: "Wireframes",
					children: [
						{ label: "Core Layout", path: "ui/wireframes/core-layout" },
						{ label: "Conversation View", path: "ui/wireframes/conversation-view" },
						{ label: "Artifact Browser", path: "ui/wireframes/artifact-browser" },
						{ label: "Dashboard", path: "ui/wireframes/dashboard" },
						{ label: "Settings & Onboarding", path: "ui/wireframes/settings-onboarding" },
					],
				},
			],
		},
		{
			label: "Development",
			children: [
				{ label: "Getting Started", path: "development/getting-started" },
				{ label: "Coding Standards", path: "development/coding-standards" },
				{ label: "Lessons", path: "development/lessons" },
			],
		},
		{
			label: "Process",
			children: [
				{ label: "Team", path: "process/team" },
				{ label: "Orchestration", path: "process/orchestration" },
				{ label: "Definition of Ready", path: "process/definition-of-ready" },
				{ label: "Definition of Done", path: "process/definition-of-done" },
				{ label: "Retrospectives", path: "process/retrospectives" },
			],
		},
		{
			label: "Research",
			children: [
				{ label: "Claude Integration", path: "research/claude-integration" },
				{ label: "Tauri v2", path: "research/tauri-v2" },
				{ label: "Frontend", path: "research/frontend" },
				{ label: "Persistence", path: "research/persistence" },
			],
		},
	];

	function handleDocClick(path: string, label: string) {
		navigationStore.openArtifact(path, ["Docs", label]);
	}
</script>

<ScrollArea.Root class="h-full">
	<div class="space-y-0.5 p-2">
		{#each docTree as section}
			{@render treeSection(section, 0)}
		{/each}
	</div>
</ScrollArea.Root>

{#snippet treeSection(node: DocNode, depth: number)}
	{#if node.children}
		<Collapsible.Root open={true}>
			<Collapsible.Trigger
				class="flex w-full items-center gap-1 rounded px-1 py-1 text-xs font-semibold uppercase tracking-wide text-muted-foreground hover:bg-accent/50"
				style="padding-left: {depth * 12 + 4}px"
			>
				<ChevronRightIcon class="h-3 w-3 transition-transform [[data-state=open]_&]:rotate-90" />
				{node.label}
			</Collapsible.Trigger>
			<Collapsible.Content>
				{#each node.children as child}
					{@render treeSection(child, depth + 1)}
				{/each}
			</Collapsible.Content>
		</Collapsible.Root>
	{:else if node.path}
		<button
			class="flex w-full items-center gap-1.5 rounded px-1 py-1 text-sm text-foreground/80 hover:bg-accent/50"
			class:bg-accent={navigationStore.selectedArtifactPath === node.path}
			class:text-accent-foreground={navigationStore.selectedArtifactPath === node.path}
			style="padding-left: {depth * 12 + 8}px"
			onclick={() => handleDocClick(node.path!, node.label)}
		>
			<FileTextIcon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
			<span class="truncate">{node.label}</span>
		</button>
	{/if}
{/snippet}
