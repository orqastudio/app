import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/svelte";
import StatusIndicator from "../StatusIndicator.svelte";

describe("StatusIndicator", () => {
	it("renders in badge mode by default", () => {
		render(StatusIndicator, {
			props: { status: "active" },
		});
		// Badge mode renders the status label (resolved via FALLBACK_STATUSES: "Active")
		expect(screen.getByText("Active")).toBeInTheDocument();
	});

	it("renders in dot mode with an svg icon", () => {
		const { container } = render(StatusIndicator, {
			props: { status: "completed", mode: "dot" },
		});
		// Dot mode renders an SVG icon (Lucide component)
		const svg = container.querySelector("svg");
		expect(svg).toBeInTheDocument();
	});

	it("renders in inline mode with status label", () => {
		render(StatusIndicator, {
			props: { status: "active", mode: "inline" },
		});
		// Inline mode shows resolved label, not raw key
		expect(screen.getByText("Active")).toBeInTheDocument();
	});

	it("renders a known status with its resolved label", () => {
		render(StatusIndicator, {
			props: { status: "completed", mode: "badge" },
		});
		expect(screen.getByText("Completed")).toBeInTheDocument();
	});

	it("falls back to raw key for unknown statuses", () => {
		render(StatusIndicator, {
			props: { status: "unknown-status", mode: "badge" },
		});
		// Unknown status: label falls back to the raw status string
		expect(screen.getByText("unknown-status")).toBeInTheDocument();
	});
});
