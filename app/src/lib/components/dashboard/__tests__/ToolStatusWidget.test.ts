/**
 * Tests for ToolStatusWidget.svelte.
 *
 * ToolStatusWidget reads cliToolStatuses from pluginStore and renders a
 * card for each tool with a Run button. When no tools are registered, it
 * renders nothing.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));

import ToolStatusWidget from "../ToolStatusWidget.svelte";

describe("ToolStatusWidget", () => {
	beforeEach(() => {
		installMockStores();
	});

	afterEach(() => {
		clearMockStores();
	});

	it("renders nothing when no tools are registered", () => {
		const { container } = render(ToolStatusWidget);
		// No card should be rendered
		expect(container.querySelector("[class*=card]")).not.toBeInTheDocument();
	});

	it("renders Plugin CLI Tools heading when tools are registered", () => {
		installMockStores({
			pluginStore: {
				cliToolStatuses: [
					{
						plugin: "claude",
						tool_key: "check",
						label: "Check Auth",
						success: true,
						summary: "Authenticated",
						last_duration_ms: 150,
					},
				],
				installed: [],
				error: null,
				loadingRegistry: false,
				loadInstalled: vi.fn(),
				loadCliToolStatuses: vi.fn().mockResolvedValue(undefined),
				listRegistry: vi.fn(),
				installFromGitHub: vi.fn(),
				installFromLocal: vi.fn(),
				uninstall: vi.fn(),
				runCliTool: vi.fn().mockResolvedValue({ exit_code: 0, stdout: "OK", stderr: "" }),
				getManifest: vi.fn(),
			},
		});
		render(ToolStatusWidget);
		expect(screen.getByText("Plugin CLI Tools")).toBeInTheDocument();
	});

	it("renders each tool's label and summary", () => {
		installMockStores({
			pluginStore: {
				cliToolStatuses: [
					{
						plugin: "claude",
						tool_key: "check",
						label: "Check Auth",
						success: true,
						summary: "All good",
						last_duration_ms: 200,
					},
					{
						plugin: "svelte",
						tool_key: "build",
						label: "Build",
						success: false,
						summary: "Build failed",
						last_duration_ms: 5000,
					},
				],
				installed: [],
				error: null,
				loadingRegistry: false,
				loadInstalled: vi.fn(),
				loadCliToolStatuses: vi.fn().mockResolvedValue(undefined),
				listRegistry: vi.fn(),
				installFromGitHub: vi.fn(),
				installFromLocal: vi.fn(),
				uninstall: vi.fn(),
				runCliTool: vi.fn().mockResolvedValue({ exit_code: 0, stdout: "OK", stderr: "" }),
				getManifest: vi.fn(),
			},
		});
		render(ToolStatusWidget);
		expect(screen.getByText("Check Auth")).toBeInTheDocument();
		expect(screen.getByText("Build")).toBeInTheDocument();
		expect(screen.getByText(/All good.*200ms/)).toBeInTheDocument();
	});

	it("renders a Run button for each tool", () => {
		installMockStores({
			pluginStore: {
				cliToolStatuses: [
					{
						plugin: "claude",
						tool_key: "check",
						label: "Check Auth",
						success: null,
						summary: null,
						last_duration_ms: null,
					},
					{
						plugin: "svelte",
						tool_key: "lint",
						label: "Lint",
						success: null,
						summary: null,
						last_duration_ms: null,
					},
				],
				installed: [],
				error: null,
				loadingRegistry: false,
				loadInstalled: vi.fn(),
				loadCliToolStatuses: vi.fn().mockResolvedValue(undefined),
				listRegistry: vi.fn(),
				installFromGitHub: vi.fn(),
				installFromLocal: vi.fn(),
				uninstall: vi.fn(),
				runCliTool: vi.fn().mockResolvedValue({ exit_code: 0, stdout: "OK", stderr: "" }),
				getManifest: vi.fn(),
			},
		});
		render(ToolStatusWidget);
		const runButtons = screen.getAllByRole("button", { name: /Run/i });
		expect(runButtons).toHaveLength(2);
	});

	it("calls pluginStore.runCliTool when Run is clicked", async () => {
		const mockRunCliTool = vi.fn().mockResolvedValue({ exit_code: 0, stdout: "OK", stderr: "" });
		installMockStores({
			pluginStore: {
				cliToolStatuses: [
					{
						plugin: "claude",
						tool_key: "check",
						label: "Check Auth",
						success: null,
						summary: null,
						last_duration_ms: null,
					},
				],
				installed: [],
				error: null,
				loadingRegistry: false,
				loadInstalled: vi.fn(),
				loadCliToolStatuses: vi.fn().mockResolvedValue(undefined),
				listRegistry: vi.fn(),
				installFromGitHub: vi.fn(),
				installFromLocal: vi.fn(),
				uninstall: vi.fn(),
				runCliTool: mockRunCliTool,
				getManifest: vi.fn(),
			},
		});
		render(ToolStatusWidget);
		const runBtn = screen.getByRole("button", { name: /Run/i });
		await fireEvent.click(runBtn);
		expect(mockRunCliTool).toHaveBeenCalledWith("claude", "check");
	});

	it("renders 'Not run yet' for tools without summary", () => {
		installMockStores({
			pluginStore: {
				cliToolStatuses: [
					{
						plugin: "claude",
						tool_key: "check",
						label: "Check Auth",
						success: null,
						summary: null,
						last_duration_ms: null,
					},
				],
				installed: [],
				error: null,
				loadingRegistry: false,
				loadInstalled: vi.fn(),
				loadCliToolStatuses: vi.fn().mockResolvedValue(undefined),
				listRegistry: vi.fn(),
				installFromGitHub: vi.fn(),
				installFromLocal: vi.fn(),
				uninstall: vi.fn(),
				runCliTool: vi.fn().mockResolvedValue({ exit_code: 0, stdout: "", stderr: "" }),
				getManifest: vi.fn(),
			},
		});
		render(ToolStatusWidget);
		expect(screen.getByText("Not run yet")).toBeInTheDocument();
	});
});
