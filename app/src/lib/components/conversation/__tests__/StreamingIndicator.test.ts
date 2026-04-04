/**
 * Tests for StreamingIndicator.svelte.
 *
 * StreamingIndicator shows animated dots with a status label while the AI
 * is responding. It accepts hasContent and toolCalls props. When hasContent
 * is true and no tools are active it should be hidden.
 */

import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("$lib/utils/tool-display", () => ({
	getActivityPhase: vi.fn().mockReturnValue("Working"),
	getEphemeralLabel: vi.fn().mockReturnValue(null),
}));

import StreamingIndicator from "../StreamingIndicator.svelte";

describe("StreamingIndicator", () => {
	it("renders with no content and no tools (default waiting state)", () => {
		render(StreamingIndicator, { props: { hasContent: false, toolCalls: [] } });
		// Should be visible — waiting phrase ends with "..."
		expect(screen.getByText(/\.\.\.$/)).toBeInTheDocument();
	});

	it("shows animated dots", () => {
		const { container } = render(StreamingIndicator, { props: { hasContent: false, toolCalls: [] } });
		// Three animated spans with animate-bounce class
		const dots = container.querySelectorAll(".animate-bounce");
		expect(dots.length).toBe(3);
	});

	it("is hidden when content is streaming and no tools are active", () => {
		// visible = !hasContent || hasActiveTools — false when hasContent=true, toolCalls=[]
		render(StreamingIndicator, { props: { hasContent: true, toolCalls: [] } });
		// The status dots and label should not be rendered
		expect(screen.queryByText(/\.\.\.$/)).not.toBeInTheDocument();
		expect(document.querySelectorAll(".animate-bounce").length).toBe(0);
	});

	it("is visible when tools are active even if content exists", () => {
		const mockToolCall = {
			toolCallId: "call-1",
			toolName: "bash",
			input: JSON.stringify({ command: "ls" }),
			output: null,
			isError: false,
			isComplete: false,
		};
		render(StreamingIndicator, { props: { hasContent: true, toolCalls: [mockToolCall] } });
		// Phase label from mock: "Working..."
		expect(screen.getByText("Working...")).toBeInTheDocument();
	});

	it("shows tool progress counter when tools are active", () => {
		const toolCalls = [
			{ toolCallId: "c1", toolName: "bash", input: "{}", output: null, isError: false, isComplete: true },
			{ toolCallId: "c2", toolName: "read", input: "{}", output: null, isError: false, isComplete: false },
		];
		render(StreamingIndicator, { props: { hasContent: false, toolCalls } });
		// 1 of 2 tools complete
		expect(screen.getByText("(1/2 tools)")).toBeInTheDocument();
	});
});
