import type { Meta, StoryObj } from "@storybook/svelte";
import Panel from "./Panel.svelte";

const meta = {
	title: "Pure/Panel",
	component: Panel,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Normal: Story = { args: { padding: "normal" } };
export const Tight: Story = { args: { padding: "tight" } };
export const Loose: Story = { args: { padding: "loose" } };
export const WithBorder: Story = { args: { padding: "normal", border: "all", rounded: "md" } };
export const MutedBackground: Story = {
	args: { padding: "normal", background: "muted", rounded: "md" },
};
export const CardBackground: Story = {
	args: { padding: "normal", background: "card", border: "all", rounded: "lg" },
};
export const TopBorder: Story = { args: { padding: "normal", border: "top" } };
export const BottomBorder: Story = { args: { padding: "normal", border: "bottom" } };
