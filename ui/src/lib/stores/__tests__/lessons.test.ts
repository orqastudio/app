import { describe, it, expect, beforeEach } from "vitest";
import { mockInvoke } from "./setup";

import { lessonStore } from "../lessons.svelte";
import type { Lesson } from "$lib/types/lessons";

const makeFakeLesson = (overrides: Partial<Lesson> = {}): Lesson => ({
	id: "IMPL-001",
	title: "Test lesson",
	category: "testing",
	status: "active",
	recurrence: 0,
	promoted_to: null,
	created: "2026-01-01T00:00:00Z",
	updated: "2026-01-01T00:00:00Z",
	body: "Lesson body",
	file_path: ".orqa/process/lessons/IMPL-001.md",
	...overrides,
});

beforeEach(() => {
	mockInvoke.mockReset();
	lessonStore.lessons = [];
	lessonStore.loading = false;
	lessonStore.error = null;
});

describe("LessonStore", () => {
	describe("initial state", () => {
		it("starts empty", () => {
			expect(lessonStore.lessons).toEqual([]);
			expect(lessonStore.loading).toBe(false);
			expect(lessonStore.error).toBeNull();
		});

		it("promotionCandidates is empty initially", () => {
			expect(lessonStore.promotionCandidates).toEqual([]);
		});
	});

	describe("loadLessons", () => {
		it("loads lessons from backend", async () => {
			const lessons = [makeFakeLesson({ id: "IMPL-001" }), makeFakeLesson({ id: "IMPL-002" })];
			mockInvoke.mockResolvedValueOnce(lessons);

			await lessonStore.loadLessons("/path/to/project");

			expect(mockInvoke).toHaveBeenCalledWith("lessons_list", { projectPath: "/path/to/project" });
			expect(lessonStore.lessons).toEqual(lessons);
			expect(lessonStore.loading).toBe(false);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Failed to read"));

			await lessonStore.loadLessons("/path");

			expect(lessonStore.error).toBe("Failed to read");
			expect(lessonStore.loading).toBe(false);
		});
	});

	describe("createLesson", () => {
		it("adds created lesson to the list", async () => {
			const newLesson = makeFakeLesson({ id: "IMPL-003", title: "New lesson" });
			mockInvoke.mockResolvedValueOnce(newLesson);

			await lessonStore.createLesson("/path", "New lesson", "testing", "Body");

			expect(mockInvoke).toHaveBeenCalledWith("lessons_create", {
				projectPath: "/path",
				title: "New lesson",
				category: "testing",
				body: "Body",
			});
			expect(lessonStore.lessons).toHaveLength(1);
			expect(lessonStore.lessons[0].id).toBe("IMPL-003");
		});

		it("sorts lessons by id after adding", async () => {
			lessonStore.lessons = [makeFakeLesson({ id: "IMPL-005" })];
			const newLesson = makeFakeLesson({ id: "IMPL-002" });
			mockInvoke.mockResolvedValueOnce(newLesson);

			await lessonStore.createLesson("/path", "Earlier", "testing", "Body");

			expect(lessonStore.lessons[0].id).toBe("IMPL-002");
			expect(lessonStore.lessons[1].id).toBe("IMPL-005");
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Create failed"));

			await lessonStore.createLesson("/path", "t", "c", "b");

			expect(lessonStore.error).toBe("Create failed");
		});
	});

	describe("incrementRecurrence", () => {
		it("updates the lesson in the list", async () => {
			const original = makeFakeLesson({ id: "IMPL-001", recurrence: 1 });
			lessonStore.lessons = [original];

			const updated = makeFakeLesson({ id: "IMPL-001", recurrence: 2 });
			mockInvoke.mockResolvedValueOnce(updated);

			await lessonStore.incrementRecurrence("/path", "IMPL-001");

			expect(mockInvoke).toHaveBeenCalledWith("lesson_increment_recurrence", {
				projectPath: "/path",
				id: "IMPL-001",
			});
			expect(lessonStore.lessons[0].recurrence).toBe(2);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Increment failed"));

			await lessonStore.incrementRecurrence("/path", "IMPL-001");

			expect(lessonStore.error).toBe("Increment failed");
		});
	});

	describe("promotionCandidates", () => {
		it("filters lessons with recurrence >= 2 and status active", () => {
			lessonStore.lessons = [
				makeFakeLesson({ id: "IMPL-001", recurrence: 1, status: "active" }),
				makeFakeLesson({ id: "IMPL-002", recurrence: 2, status: "active" }),
				makeFakeLesson({ id: "IMPL-003", recurrence: 3, status: "promoted" }),
				makeFakeLesson({ id: "IMPL-004", recurrence: 5, status: "active" }),
			];

			const candidates = lessonStore.promotionCandidates;
			expect(candidates).toHaveLength(2);
			expect(candidates.map((c) => c.id)).toEqual(["IMPL-002", "IMPL-004"]);
		});
	});
});
