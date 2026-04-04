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

	function handleSelect(lesson: Lesson) {
		selectedLesson = lesson;
	}

	async function handleIncrementRecurrence(id: string) {
		if (!projectPath) return;
		await lessonStore.incrementRecurrence(projectPath, id);
		// Refresh the selected lesson state from the updated store
		const updated = lessonStore.lessons.find((l) => l.id === id);
		if (updated) {
			selectedLesson = updated;
		}
	}

	function handleRetry() {
		if (projectPath) {
			lessonStore.loadLessons(projectPath);
		}
	}
</script>

<HStack gap={0} style="height: 100%;">
	<!-- Lesson list sidebar (240px) -->
	<div style="width: 15rem; flex-shrink: 0; overflow: hidden; border-right: 1px solid hsl(var(--border));">
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
