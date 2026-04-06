import type { Meta, StoryObj } from "@storybook/svelte";
import ContextTable from "./ContextTable.svelte";

const meta = {
	title: "Pure/ContextTable",
	component: ContextTable,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const BasicEntries: Story = {
	args: {
		entries: [
			{ key: "level", value: "error" },
			{ key: "service", value: "auth-service" },
			{ key: "environment", value: "production" },
			{ key: "host", value: "api-prod-03" },
		],
	},
};

export const WithClickableValue: Story = {
	args: {
		entries: [
			{ key: "correlation_id", value: "abc-123-def-456", copyable: true },
			{ key: "request_id", value: "req-789-xyz" },
		],
		onValueClick: (key: string, value: string) => console.log("clicked", key, value),
	},
};

export const Empty: Story = {
	args: {
		entries: [],
	},
};
