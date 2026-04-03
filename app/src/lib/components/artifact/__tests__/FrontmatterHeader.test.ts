/**
 * Tests for FrontmatterHeader.svelte.
 *
 * FrontmatterHeader receives a metadata record and artifact type as props
 * and renders a structured metadata card. No store dependencies — pure prop
 * rendering so no IPC mocking is required.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));

import FrontmatterHeader from "../FrontmatterHeader.svelte";

describe("FrontmatterHeader", () => {
	beforeEach(() => {
		installMockStores();
	});

	afterEach(() => {
		clearMockStores();
	});

	it("renders the title field", () => {
		render(FrontmatterHeader, {
			props: {
				metadata: { title: "My Epic", status: "active" },
				artifactType: "epic",
			},
		});
		expect(screen.getByText("My Epic")).toBeInTheDocument();
	});

	it("renders the description field", () => {
		render(FrontmatterHeader, {
			props: {
				metadata: { title: "My Epic", description: "An important epic" },
				artifactType: "epic",
			},
		});
		expect(screen.getByText("An important epic")).toBeInTheDocument();
	});

	it("renders the artifact type and ID", () => {
		render(FrontmatterHeader, {
			props: {
				metadata: { title: "My Task", id: "TASK-001", status: "active" },
				artifactType: "task",
			},
		});
		expect(screen.getByText(/task.*TASK-001/i)).toBeInTheDocument();
	});

	it("renders status badge", () => {
		render(FrontmatterHeader, {
			props: {
				metadata: { title: "My Task", status: "in-progress" },
				artifactType: "task",
			},
		});
		// StatusIndicator renders a badge — check any text that contains the status
		expect(screen.getByText(/in.progress/i)).toBeInTheDocument();
	});

	it("renders priority badge when priority is set", () => {
		render(FrontmatterHeader, {
			props: {
				metadata: { title: "My Task", priority: "high" },
				artifactType: "task",
			},
		});
		expect(screen.getByText(/high/i)).toBeInTheDocument();
	});

	it("renders generic string fields in the body", () => {
		render(FrontmatterHeader, {
			props: {
				metadata: { title: "My Epic", owner: "Alice" },
				artifactType: "epic",
			},
		});
		expect(screen.getByText("Alice")).toBeInTheDocument();
		expect(screen.getByText("Owner")).toBeInTheDocument();
	});

	it("renders chip fields as badges", () => {
		render(FrontmatterHeader, {
			props: {
				metadata: { title: "My Epic", tags: ["alpha", "beta"] },
				artifactType: "epic",
			},
		});
		expect(screen.getByText("alpha")).toBeInTheDocument();
		expect(screen.getByText("beta")).toBeInTheDocument();
	});

	it("skips null or empty values", () => {
		render(FrontmatterHeader, {
			props: {
				metadata: { title: "My Task", description: null, owner: "" },
				artifactType: "task",
			},
		});
		// owner and description should not render empty rows
		expect(screen.queryByText("Owner")).not.toBeInTheDocument();
	});

	it("renders title-only view without body when no metadata fields", () => {
		render(FrontmatterHeader, {
			props: {
				metadata: { title: "Clean Task", description: "Short description" },
				artifactType: "task",
			},
		});
		expect(screen.getByText("Clean Task")).toBeInTheDocument();
		expect(screen.getByText("Short description")).toBeInTheDocument();
	});
});
