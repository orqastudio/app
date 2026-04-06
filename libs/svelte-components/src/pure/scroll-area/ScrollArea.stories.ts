import type { Meta, StoryObj } from "@storybook/svelte";
import { ScrollArea } from "./index.js";

const meta = {
	title: "Pure/ScrollArea",
	component: ScrollArea,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Vertical: Story = { args: { orientation: "vertical", maxHeight: "md" } };
export const Horizontal: Story = { args: { orientation: "horizontal", maxHeight: "sm" } };
export const Both: Story = { args: { orientation: "both", maxHeight: "md" } };
export const FixedPixelHeight: Story = { args: { orientation: "vertical", heightPx: 280 } };
