<script lang="ts">
	import ArrowRightIcon from "@lucide/svelte/icons/arrow-right";

	let {
		artifactType,
		metadata,
	}: {
		artifactType: string;
		metadata: Record<string, unknown>;
	} = $props();

	/** Terminal states where no actions are needed. */
	const TERMINAL_STATES = new Set([
		"done",
		"complete",
		"promoted",
		"accepted",
		"archived",
		"superseded",
		"surpassed",
	]);

	/** Infer actions based on artifact type and status. */
	const actions = $derived.by((): string[] => {
		const status = String(metadata["status"] ?? "").toLowerCase();
		const type = artifactType.toLowerCase();

		if (TERMINAL_STATES.has(status)) return [];

		if (type === "task") {
			if (status === "todo") {
				return ["Verify depends-on tasks are done, assign to agent"];
			}
			if (status === "in-progress") {
				return [
					"Complete acceptance criteria, request reviewer verification",
				];
			}
		}

		if (type === "epic") {
			if (status === "draft") {
				const docsRequired = metadata["docs-required"];
				if (Array.isArray(docsRequired) && docsRequired.length > 0) {
					return [
						`Satisfy docs-required gate: ${docsRequired.join(", ")}`,
					];
				}
				return ["Satisfy docs-required gate"];
			}
			if (status === "ready") {
				return ["Create worktree, assign implementation agent"];
			}
			if (status === "in-progress") {
				return ["Complete implementation, submit for review"];
			}
			if (status === "review") {
				return [
					"Pass verification gates (code-reviewer, qa-tester, ux-reviewer)",
				];
			}
		}

		if (type === "idea") {
			if (status === "captured") {
				return ["Approve for investigation to begin exploring"];
			}
			if (status === "exploring") {
				return ["Complete research-needed items to shape"];
			}
			if (status === "shaped") {
				return ["Approve promotion to epic"];
			}
		}

		if (type === "lesson") {
			if (status === "active") {
				const recurrence = Number(metadata["recurrence"] ?? 0);
				if (recurrence >= 2) {
					return ["Promote to rule or skill update"];
				}
			}
			if (status === "recurring") {
				return ["Promote to rule or skill update"];
			}
		}

		if (type === "decision") {
			if (status === "proposed") {
				return ["Review and approve"];
			}
		}

		if (type === "research") {
			if (status === "draft") {
				return ["Complete investigation and document findings"];
			}
		}

		if (type === "milestone") {
			if (status === "planning") {
				return ["Create epics with status ready or later"];
			}
			if (status === "active") {
				return ["Complete all P1 epics"];
			}
		}

		return [];
	});
</script>

{#if actions.length > 0}
	<div class="mb-4 rounded-lg border border-amber-500/30 bg-amber-500/5 px-4 py-3">
		<p class="mb-1.5 text-xs font-semibold uppercase tracking-wider text-amber-600 dark:text-amber-400">
			Actions Needed
		</p>
		<ul class="space-y-1">
			{#each actions as action, i (i)}
				<li class="flex items-start gap-2 text-xs text-foreground">
					<ArrowRightIcon class="mt-0.5 h-3 w-3 shrink-0 text-amber-500" />
					<span>{action}</span>
				</li>
			{/each}
		</ul>
	</div>
{/if}
