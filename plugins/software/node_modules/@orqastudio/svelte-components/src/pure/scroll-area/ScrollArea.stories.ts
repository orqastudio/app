import type { Meta, StoryObj } from "@storybook/svelte";
import { ScrollArea } from "./index.js";

const meta = {
	title: "Pure/ScrollArea",
	component: ScrollArea,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Vertical: Story = { args: { orientation: "vertical", class: "h-48 w-64" } };
export const Horizontal: Story = { args: { orientation: "horizontal", class: "h-24 w-48" } };
export const Both: Story = { args: { orientation: "both", class: "h-48 w-48" } };
