import type { Meta, StoryObj } from "@storybook/svelte";
import EventDrawer from "./EventDrawer.svelte";

const meta = {
	title: "Pure/EventDrawer",
	component: EventDrawer,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

/** Mock log event used across stories. */
const mockEvent = {
	id: 1001,
	level: "Error",
	source: "Daemon",
	message: "Graph traversal failed: circular dependency detected in workflow",
	timestamp: Date.now(),
	category: "graph-engine",
};

/** Drawer open on the Stack tab with a selected event. */
export const OpenStackTab: Story = {
	args: {
		open: true,
		event: mockEvent,
		activeTab: "stack",
	},
};

/** Drawer open on the Context tab. */
export const OpenContextTab: Story = {
	args: {
		open: true,
		event: mockEvent,
		activeTab: "context",
	},
};

/** Drawer open on the Raw tab. */
export const OpenRawTab: Story = {
	args: {
		open: true,
		event: mockEvent,
		activeTab: "raw",
	},
};

/** Drawer in closed state — renders nothing. */
export const Closed: Story = {
	args: {
		open: false,
		event: mockEvent,
		activeTab: "stack",
	},
};

/** Drawer open with no event selected. */
export const NoEvent: Story = {
	args: {
		open: true,
		event: null,
		activeTab: "stack",
	},
};
