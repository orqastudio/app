<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import { Badge } from "$lib/components/ui/badge";
	import Breadcrumb from "./Breadcrumb.svelte";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import type { ComplianceStatus } from "$lib/types";

	const complianceVariant: Record<ComplianceStatus, "default" | "secondary" | "destructive" | "outline"> = {
		compliant: "default",
		non_compliant: "destructive",
		unknown: "secondary",
		error: "destructive",
	};

	const artifact = $derived(artifactStore.activeArtifact);
	const breadcrumbs = $derived(navigationStore.breadcrumbs);
</script>

<div class="flex h-full flex-col">
	<!-- Breadcrumb bar -->
	<div class="flex items-center justify-between border-b border-border px-4 py-2">
		<Breadcrumb items={breadcrumbs} />
	</div>

	<!-- Content -->
	{#if artifactStore.loading}
		<div class="flex flex-1 items-center justify-center">
			<LoadingSpinner />
		</div>
	{:else if artifactStore.error}
		<div class="flex flex-1 items-center justify-center px-4">
			<ErrorDisplay message={artifactStore.error} />
		</div>
	{:else if artifact}
		<ScrollArea.Root class="flex-1">
			<div class="p-6">
				<!-- Metadata header -->
				<div class="mb-6 rounded-lg border border-border bg-muted/20 p-4">
					<div class="flex items-start justify-between">
						<h1 class="text-xl font-bold">{artifact.name}</h1>
						<Badge variant={complianceVariant[artifact.compliance_status]}>
							{artifact.compliance_status}
						</Badge>
					</div>
					{#if artifact.description}
						<p class="mt-1 text-sm text-muted-foreground">{artifact.description}</p>
					{/if}
					<div class="mt-3 flex flex-wrap gap-4 text-xs text-muted-foreground">
						<span>Type: <strong>{artifact.artifact_type}</strong></span>
						<span>Path: <strong>{artifact.rel_path}</strong></span>
						{#if artifact.file_modified_at}
							<span>Modified: <strong>{new Date(artifact.file_modified_at).toLocaleDateString()}</strong></span>
						{/if}
					</div>
				</div>

				<!-- Rendered content -->
				<MarkdownRenderer content={artifact.content} />
			</div>
		</ScrollArea.Root>
	{:else}
		<div class="flex flex-1 items-center justify-center text-sm text-muted-foreground">
			Select an artifact to view its contents
		</div>
	{/if}
</div>
