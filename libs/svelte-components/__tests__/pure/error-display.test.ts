// Tests for ErrorDisplay — renders error message, optional retry button.
import { describe, it, expect, vi } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import ErrorDisplay from "../../src/pure/error-display/ErrorDisplay.svelte";

describe("ErrorDisplay", () => {
	it("renders the error message", () => {
		const { getByText } = render(ErrorDisplay, { props: { message: "Something went wrong" } });
		expect(getByText("Something went wrong")).toBeInTheDocument();
	});

	it("renders a Retry button when onRetry is provided", () => {
		const { getByRole } = render(ErrorDisplay, {
			props: { message: "Error", onRetry: vi.fn() },
		});
		expect(getByRole("button", { name: "Retry" })).toBeInTheDocument();
	});

	it("calls onRetry when the Retry button is clicked", async () => {
		const onRetry = vi.fn();
		const { getByRole } = render(ErrorDisplay, {
			props: { message: "Error", onRetry },
		});
		await fireEvent.click(getByRole("button", { name: "Retry" }));
		expect(onRetry).toHaveBeenCalledOnce();
	});

	it("does not render a Retry button when onRetry is omitted", () => {
		const { queryByRole } = render(ErrorDisplay, { props: { message: "Error" } });
		expect(queryByRole("button")).toBeNull();
	});
});
