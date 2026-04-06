import type { Meta, StoryObj } from "@storybook/svelte";
import TraceTimeline from "./TraceTimeline.svelte";

const meta = {
	title: "Pure/TraceTimeline",
	component: TraceTimeline,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

/** Multi-component trace: events spread across Daemon, App, and MCP over ~500ms. */
export const MultiComponent: Story = {
	args: {
		events: [
			{
				id: 1,
				timestamp: 1000,
				source: "Daemon",
				level: "Info",
				message: "Request received",
				correlation_id: "abc-123",
			},
			{
				id: 2,
				timestamp: 1050,
				source: "App",
				level: "Info",
				message: "IPC dispatch",
				correlation_id: "abc-123",
			},
			{
				id: 3,
				timestamp: 1120,
				source: "MCP",
				level: "Debug",
				message: "Tool call start",
				correlation_id: "abc-123",
			},
			{
				id: 4,
				timestamp: 1300,
				source: "MCP",
				level: "Info",
				message: "Tool call complete",
				correlation_id: "abc-123",
			},
			{
				id: 5,
				timestamp: 1420,
				source: "App",
				level: "Warn",
				message: "Slow response detected",
				correlation_id: "abc-123",
			},
			{
				id: 6,
				timestamp: 1500,
				source: "Daemon",
				level: "Info",
				message: "Response sent",
				correlation_id: "abc-123",
			},
		],
	},
};

/** Single-component trace: all events from one source — no swim-lane separation. */
export const SingleComponent: Story = {
	args: {
		events: [
			{
				id: 10,
				timestamp: 2000,
				source: "Search",
				level: "Debug",
				message: "Index query start",
				correlation_id: "def-456",
			},
			{
				id: 11,
				timestamp: 2080,
				source: "Search",
				level: "Info",
				message: "Embeddings computed",
				correlation_id: "def-456",
			},
			{
				id: 12,
				timestamp: 2200,
				source: "Search",
				level: "Error",
				message: "Similarity threshold not met",
				correlation_id: "def-456",
			},
		],
	},
};

/** Empty state: no correlation ID selected yet. */
export const Empty: Story = {
	args: {
		events: [],
	},
};
