import type { Meta, StoryObj } from "@storybook/svelte";
import RawJson from "./RawJson.svelte";

const meta = {
	title: "Pure/RawJson",
	component: RawJson,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const SimpleObject: Story = {
	args: {
		data: { level: "error", message: "connection refused", code: 503 },
	},
};

export const NestedObject: Story = {
	args: {
		data: {
			event: {
				id: "evt-001",
				timestamp: "2026-04-06T12:00:00Z",
				context: {
					service: "auth-service",
					host: "api-prod-03",
					tags: ["critical", "prod"],
				},
			},
			meta: { version: 2, source: "filebeat" },
		},
	},
};

export const Null: Story = {
	args: {
		data: null,
	},
};
