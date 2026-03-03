<script lang="ts">
	import { Badge } from "$lib/components/ui/badge";
	import UsersIcon from "@lucide/svelte/icons/users";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import type { ArtifactSummary, ArtifactType, ComplianceStatus } from "$lib/types";
	import type { Component } from "svelte";

	let {
		artifact,
		selected = false,
		onclick,
	}: {
		artifact: ArtifactSummary;
		selected?: boolean;
		onclick: () => void;
	} = $props();

	const typeIcons: Record<ArtifactType, Component> = {
		agent: UsersIcon,
		rule: ShieldIcon,
		skill: ZapIcon,
		hook: GitBranchIcon,
		doc: FileTextIcon,
	};

	const typeLabels: Record<ArtifactType, string> = {
		agent: "Agent",
		rule: "Rule",
		skill: "Skill",
		hook: "Hook",
		doc: "Doc",
	};

	const complianceVariant: Record<ComplianceStatus, "default" | "secondary" | "destructive" | "outline"> = {
		compliant: "default",
		non_compliant: "destructive",
		unknown: "secondary",
		error: "destructive",
	};

	const Icon = $derived(typeIcons[artifact.artifact_type]);
</script>

<button
	class="flex w-full items-center gap-3 rounded-md border border-transparent px-3 py-2.5 text-left transition-colors hover:bg-accent/50"
	class:border-border={selected}
	class:bg-accent={selected}
	{onclick}
>
	<Icon class="h-5 w-5 shrink-0 text-muted-foreground" />

	<div class="min-w-0 flex-1">
		<div class="flex items-center gap-2">
			<span class="truncate text-sm font-medium">{artifact.name}</span>
			<Badge variant="outline" class="text-[10px] px-1.5 py-0 shrink-0">
				{typeLabels[artifact.artifact_type]}
			</Badge>
		</div>
		{#if artifact.description}
			<p class="mt-0.5 line-clamp-1 text-xs text-muted-foreground">
				{artifact.description}
			</p>
		{/if}
	</div>

	<Badge variant={complianceVariant[artifact.compliance_status]} class="shrink-0 text-[10px] px-1.5 py-0">
		{artifact.compliance_status}
	</Badge>
</button>
