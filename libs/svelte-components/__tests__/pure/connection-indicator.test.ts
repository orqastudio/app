// Tests for ConnectionIndicator — renders correct state labels and dot colors.
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import ConnectionIndicator from "../../src/pure/connection-indicator/ConnectionIndicator.svelte";

describe("ConnectionIndicator", () => {
	it("shows 'Connected' label for connected state", () => {
		const { getByText } = render(ConnectionIndicator, { props: { state: "connected" } });
		expect(getByText("Connected")).toBeInTheDocument();
	});

	it("shows 'Reconnecting...' label for reconnecting state", () => {
		const { getByText } = render(ConnectionIndicator, { props: { state: "reconnecting" } });
		expect(getByText("Reconnecting...")).toBeInTheDocument();
	});

	it("shows 'Disconnected' label for disconnected state", () => {
		const { getByText } = render(ConnectionIndicator, { props: { state: "disconnected" } });
		expect(getByText("Disconnected")).toBeInTheDocument();
	});

	it("shows 'Waiting...' label for waiting state (default)", () => {
		const { getByText } = render(ConnectionIndicator);
		expect(getByText("Waiting...")).toBeInTheDocument();
	});

	it("renders a green dot for connected state", () => {
		const { container } = render(ConnectionIndicator, { props: { state: "connected" } });
		const dot = container.querySelector("span.rounded-full");
		expect(dot!.className).toContain("bg-green-500");
	});

	it("renders a yellow dot for reconnecting state", () => {
		const { container } = render(ConnectionIndicator, { props: { state: "reconnecting" } });
		const dot = container.querySelector("span.rounded-full");
		expect(dot!.className).toContain("bg-yellow-500");
	});

	it("renders a red dot for disconnected state", () => {
		const { container } = render(ConnectionIndicator, { props: { state: "disconnected" } });
		const dot = container.querySelector("span.rounded-full");
		expect(dot!.className).toContain("bg-red-500");
	});

	it("shows a custom label when provided, overriding the default", () => {
		const { getByText } = render(ConnectionIndicator, {
			props: { state: "connected", label: "Live" },
		});
		expect(getByText("Live")).toBeInTheDocument();
	});
});
