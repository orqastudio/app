import type { Meta, StoryObj } from "@storybook/svelte";
import Kbd from "./Kbd.svelte";

const meta = {
	title: "Pure/Kbd",
	component: Kbd,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {};
