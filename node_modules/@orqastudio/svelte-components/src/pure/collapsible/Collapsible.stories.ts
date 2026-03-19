import type { Meta, StoryObj } from "@storybook/svelte";
import Collapsible from "./SimpleCollapsible.svelte";

const meta = {
	title: "Pure/Collapsible",
	component: Collapsible,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { open: false } };
export const OpenByDefault: Story = { args: { open: true } };
