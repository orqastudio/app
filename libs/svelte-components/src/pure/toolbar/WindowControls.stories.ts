import type { Meta, StoryObj } from "@storybook/svelte";
import WindowControls from "./WindowControls.svelte";

const meta = {
	title: "Pure/WindowControls",
	component: WindowControls,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: {} };
