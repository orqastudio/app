import type { Meta, StoryObj } from "@storybook/svelte";
import Callout from "./Callout.svelte";

const meta = {
	title: "Pure/Callout",
	component: Callout,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Info: Story = { args: { tone: "info", iconName: "info" } };
export const Warning: Story = { args: { tone: "warning", iconName: "alert-triangle" } };
export const Success: Story = { args: { tone: "success", iconName: "check-circle" } };
export const Destructive: Story = { args: { tone: "destructive", iconName: "x-circle" } };
export const Muted: Story = { args: { tone: "muted", iconName: "external-link" } };
export const Dashed: Story = {
	args: { tone: "warning", border: "dashed", iconName: "alert-triangle" },
};
export const Compact: Story = { args: { tone: "info", density: "compact", iconName: "info" } };
