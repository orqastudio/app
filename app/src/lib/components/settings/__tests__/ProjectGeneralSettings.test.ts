/**
 * Tests for ProjectGeneralSettings.svelte.
 *
 * ProjectGeneralSettings is a pure prop-based form component. It renders
 * name and description fields, and calls onSave when fields lose focus.
 * No store dependencies — Tauri dialog mock is required for icon upload.
 */

import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";

// vi.mock calls are hoisted — factories must not reference top-level variables.
// Use vi.mocked() after import to get typed references.
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn().mockResolvedValue(null) }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));

import ProjectGeneralSettings from "../ProjectGeneralSettings.svelte";
import { open } from "@tauri-apps/plugin-dialog";

const baseSettings = {
	name: "My Project",
	description: "A great project",
};

describe("ProjectGeneralSettings", () => {
	it("renders the General heading", () => {
		render(ProjectGeneralSettings, {
			props: {
				settings: baseSettings as any,
				onSave: vi.fn(),
				iconDataUrl: null,
				onUploadIcon: vi.fn(),
				onRemoveIcon: vi.fn(),
			},
		});
		expect(screen.getByText("General")).toBeInTheDocument();
	});

	it("renders the name field with current value", () => {
		render(ProjectGeneralSettings, {
			props: {
				settings: baseSettings as any,
				onSave: vi.fn(),
				iconDataUrl: null,
				onUploadIcon: vi.fn(),
				onRemoveIcon: vi.fn(),
			},
		});
		const input = screen.getByLabelText(/Name/i);
		expect(input).toHaveValue("My Project");
	});

	it("renders the description textarea with current value", () => {
		render(ProjectGeneralSettings, {
			props: {
				settings: baseSettings as any,
				onSave: vi.fn(),
				iconDataUrl: null,
				onUploadIcon: vi.fn(),
				onRemoveIcon: vi.fn(),
			},
		});
		const textarea = screen.getByLabelText(/Description/i);
		expect(textarea).toHaveValue("A great project");
	});

	it("calls onSave with updated name when name input loses focus", async () => {
		const onSave = vi.fn();
		render(ProjectGeneralSettings, {
			props: {
				settings: baseSettings as any,
				onSave,
				iconDataUrl: null,
				onUploadIcon: vi.fn(),
				onRemoveIcon: vi.fn(),
			},
		});
		const input = screen.getByLabelText(/Name/i);
		await fireEvent.input(input, { target: { value: "Updated Project" } });
		await fireEvent.blur(input);
		expect(onSave).toHaveBeenCalledWith(expect.objectContaining({ name: "Updated Project" }));
	});

	it("renders Upload button when no icon is set", () => {
		render(ProjectGeneralSettings, {
			props: {
				settings: baseSettings as any,
				onSave: vi.fn(),
				iconDataUrl: null,
				onUploadIcon: vi.fn(),
				onRemoveIcon: vi.fn(),
			},
		});
		expect(screen.getByRole("button", { name: /Upload/i })).toBeInTheDocument();
	});

	it("renders Change and Remove buttons when icon is set", () => {
		render(ProjectGeneralSettings, {
			props: {
				settings: baseSettings as any,
				onSave: vi.fn(),
				iconDataUrl: "data:image/png;base64,icon",
				onUploadIcon: vi.fn(),
				onRemoveIcon: vi.fn(),
			},
		});
		expect(screen.getByRole("button", { name: /Change/i })).toBeInTheDocument();
		expect(screen.getByRole("button", { name: /Remove/i })).toBeInTheDocument();
	});

	it("calls onRemoveIcon when Remove is clicked", async () => {
		const onRemoveIcon = vi.fn();
		render(ProjectGeneralSettings, {
			props: {
				settings: baseSettings as any,
				onSave: vi.fn(),
				iconDataUrl: "data:image/png;base64,icon",
				onUploadIcon: vi.fn(),
				onRemoveIcon,
			},
		});
		await fireEvent.click(screen.getByRole("button", { name: /Remove/i }));
		expect(onRemoveIcon).toHaveBeenCalled();
	});

	it("opens file dialog when Change button is clicked", async () => {
		render(ProjectGeneralSettings, {
			props: {
				settings: baseSettings as any,
				onSave: vi.fn(),
				iconDataUrl: "data:image/png;base64,icon",
				onUploadIcon: vi.fn(),
				onRemoveIcon: vi.fn(),
			},
		});
		await fireEvent.click(screen.getByRole("button", { name: /Change/i }));
		expect(vi.mocked(open)).toHaveBeenCalled();
	});
});
