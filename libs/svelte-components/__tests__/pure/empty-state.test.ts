// Tests for EmptyState — renders title, optional description, optional action button.
import { describe, it, expect, vi } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import EmptyState from "../../src/pure/empty-state/EmptyState.svelte";

describe("EmptyState", () => {
	it("renders the title", () => {
		const { getByText } = render(EmptyState, { props: { title: "Nothing here yet" } });
		expect(getByText("Nothing here yet")).toBeInTheDocument();
	});

	it("renders the description when provided", () => {
		const { getByText } = render(EmptyState, {
			props: { title: "Empty", description: "Try adding an item." },
		});
		expect(getByText("Try adding an item.")).toBeInTheDocument();
	});

	it("does not render description when omitted", () => {
		const { queryByText } = render(EmptyState, { props: { title: "Empty" } });
		expect(queryByText("Try adding an item.")).toBeNull();
	});

	it("renders the action button with the provided label", () => {
		const { getByRole } = render(EmptyState, {
			props: {
				title: "Empty",
				action: { label: "Add item", onclick: vi.fn() },
			},
		});
		expect(getByRole("button", { name: "Add item" })).toBeInTheDocument();
	});

	it("calls the action onclick when the button is clicked", async () => {
		const onclick = vi.fn();
		const { getByRole } = render(EmptyState, {
			props: {
				title: "Empty",
				action: { label: "Add item", onclick },
			},
		});
		await fireEvent.click(getByRole("button", { name: "Add item" }));
		expect(onclick).toHaveBeenCalledOnce();
	});

	it("does not render the action button when action prop is omitted", () => {
		const { queryByRole } = render(EmptyState, { props: { title: "Empty" } });
		expect(queryByRole("button")).toBeNull();
	});
});
