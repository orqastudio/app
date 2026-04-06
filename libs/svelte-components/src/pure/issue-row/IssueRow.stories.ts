import type { Meta, StoryObj } from "@storybook/svelte";
import IssueRow from "./IssueRow.svelte";

const meta = {
	title: "Pure/IssueRow",
	component: IssueRow,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

/** Shared sparkline data for stories. */
const sampleBuckets = [2, 4, 3, 8, 5, 12, 9, 15, 10, 7];

export const Default: Story = {
	args: {
		fingerprint: "abc123",
		title: "Unhandled promise rejection in fetchUserData",
		component: "auth/UserService",
		level: "Info",
		first_seen: Date.now() - 7 * 24 * 60 * 60 * 1000,
		last_seen: Date.now() - 5 * 60 * 1000,
		count: 12,
		sparkline_buckets: sampleBuckets,
		onclick: () => console.log("clicked"),
	},
};

export const ErrorHighCount: Story = {
	args: {
		fingerprint: "def456",
		title: "TypeError: Cannot read properties of undefined (reading 'id')",
		component: "core/GraphEngine",
		level: "Error",
		first_seen: Date.now() - 30 * 24 * 60 * 60 * 1000,
		last_seen: Date.now() - 30 * 1000,
		count: 1432,
		sparkline_buckets: [5, 8, 12, 20, 35, 50, 80, 120, 200, 300],
		onclick: () => console.log("clicked"),
	},
};

export const WarnRow: Story = {
	args: {
		fingerprint: "ghi789",
		title: "Slow query detected: schema traversal exceeded 500ms",
		component: "db/QueryPlanner",
		level: "Warn",
		first_seen: Date.now() - 3 * 24 * 60 * 60 * 1000,
		last_seen: Date.now() - 2 * 60 * 60 * 1000,
		count: 87,
		sparkline_buckets: [10, 15, 8, 20, 18, 12, 25, 22, 30, 15],
		onclick: () => console.log("clicked"),
	},
};

export const Selected: Story = {
	args: {
		fingerprint: "jkl012",
		title: "Missing required field: workflow.state not set",
		component: "plugins/WorkflowPlugin",
		level: "Warn",
		first_seen: Date.now() - 1 * 24 * 60 * 60 * 1000,
		last_seen: Date.now() - 10 * 60 * 1000,
		count: 34,
		sparkline_buckets: [3, 5, 4, 7, 6, 8, 10, 9, 12, 11],
		selected: true,
		onclick: () => console.log("clicked"),
	},
};

export const LongTitle: Story = {
	args: {
		fingerprint: "mno345",
		title:
			"ReferenceError: Cannot access 'connectorConfig' before initialization — this error occurs during hot-reload when the connector plugin re-registers its config handlers in the wrong order",
		component: "plugins/ConnectorPlugin/ConfigManager",
		level: "Error",
		first_seen: Date.now() - 60 * 60 * 1000,
		last_seen: Date.now() - 45 * 1000,
		count: 5,
		sparkline_buckets: [1, 1, 2, 3, 2, 4, 5, 4, 6, 5],
		onclick: () => console.log("clicked"),
	},
};
