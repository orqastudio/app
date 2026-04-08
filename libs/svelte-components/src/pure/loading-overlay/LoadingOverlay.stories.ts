import type { Meta, StoryObj } from "@storybook/svelte";
import LoadingOverlay from "./LoadingOverlay.svelte";

const meta = {
	title: "Pure/LoadingOverlay",
	component: LoadingOverlay,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

// LoadingOverlay is absolute-positioned — it covers a positioned ancestor.
// In Storybook it renders inline; the backdrop-blur and background are visible.
export const Default: Story = { args: {} };
