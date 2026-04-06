<script lang="ts">
	import { onMount } from "svelte";
	import LessonList from "./LessonList.svelte";
	import LessonViewer from "./LessonViewer.svelte";
	import { getStores } from "@orqastudio/sdk";
	import { Caption, HStack, Center } from "@orqastudio/svelte-components/pure";

	const { lessonStore, projectStore } = getStores();
	import type { Lesson } from "@orqastudio/types";

	let selectedLesson = $state<Lesson | null>(null);

	const projectPath = $derived(projectStore.projectPath);

	onMount(() => {
		if (projectPath) {
			lessonStore.loadLessons(projectPath);
		}
	});

	/**
	 * Set the selected lesson for display in the viewer panel.
	 * @param lesson - The lesson object the user clicked in the list.
	 */
	function handleSelect(lesson: Lesson) {
		selectedLesson = lesson;
	}

	/**
	 * Increment the recurrence count for a lesson and refresh the selected lesson from the store.
	 * @param id - The unique identifier of the lesson whose recurrence should be incremented.
	 */
	async function handleIncrementRecurrence(id: string) {
		if (!projectPath) return;
		await lessonStore.incrementRecurrence(projectPath, id);
		// Refresh the selected lesson state from the updated store
		const updated = lessonStore.lessons.find((l) => l.id === id);
		if (updated) {
			selectedLesson = updated;
		}
	}

	/** Retry loading lessons from disk when a previous load failed. */
	function handleRetry() {
		if (projectPath) {
			lessonStore.loadLessons(projectPath);
		}
	}
</script>

<HStack gap={0} height="full">
	<!-- Lesson list sidebar (240px); border-right requires raw div — no ORQA primitive supports single-side border without padding -->
	<div
		style="width: 15rem; flex-shrink: 0; overflow: hidden; border-right: 1px solid hsl(var(--border));"
	>
		<LessonList
			lessons={lessonStore.lessons}
			loading={lessonStore.loading}
			error={lessonStore.error}
			selectedId={selectedLesson?.id ?? null}
			onSelect={handleSelect}
			onRetry={handleRetry}
		/>
	</div>

	<!-- Lesson viewer -->
	<div style="min-width: 0; flex: 1; overflow: hidden;">
		{#if selectedLesson}
			<LessonViewer lesson={selectedLesson} onIncrementRecurrence={handleIncrementRecurrence} />
		{:else}
			<Center full>
				<Caption>Select a lesson to view it</Caption>
			</Center>
		{/if}
	</div>
</HStack>
