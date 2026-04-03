// Tests for NavItem — renders label, badge, active state, collapsible toggle.
import { describe, it, expect, vi } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import NavItem from "../../src/pure/nav-item/NavItem.svelte";

describe("NavItem", () => {
	it("renders the label text", () => {
		const { getByText } = render(NavItem, { props: { label: "Dashboard" } });
		expect(getByText("Dashboard")).toBeInTheDocument();
	});

	it("renders as a button", () => {
		const { getByRole } = render(NavItem, { props: { label: "Dashboard" } });
		expect(getByRole("button")).toBeInTheDocument();
	});

	it("renders the badge when provided", () => {
		const { getByText } = render(NavItem, { props: { label: "Epics", badge: 5 } });
		expect(getByText("5")).toBeInTheDocument();
	});

	it("does not render a badge when badge prop is omitted", () => {
		const { queryByText } = render(NavItem, { props: { label: "Epics" } });
		// "Epics" is rendered but there should be no badge number
		expect(queryByText("0")).toBeNull();
	});

	it("applies active styling when active=true", () => {
		const { getByRole } = render(NavItem, { props: { label: "Dashboard", active: true } });
		// Active items get the 'bg-accent' class
		expect(getByRole("button").className).toContain("bg-accent");
	});

	it("does not apply active styling when active=false", () => {
		const { getByRole } = render(NavItem, { props: { label: "Dashboard", active: false } });
		// The button gets hover:bg-accent/50 always; active adds bg-accent as a standalone class
		const cls = getByRole("button").className;
		// When inactive, 'bg-accent' should not appear without a prefix (i.e. not as a bare class)
		expect(cls).not.toContain("text-accent-foreground");
	});

	it("calls onclick when clicked", async () => {
		const onclick = vi.fn();
		const { getByRole } = render(NavItem, { props: { label: "Dashboard", onclick } });
		await fireEvent.click(getByRole("button"));
		expect(onclick).toHaveBeenCalledOnce();
	});

	it("shows a chevron when collapsible=true", () => {
		const { container } = render(NavItem, { props: { label: "Section", collapsible: true } });
		// ChevronRightIcon renders as an svg
		expect(container.querySelector("svg")).not.toBeNull();
	});
});
