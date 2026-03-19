import type { Meta, StoryObj } from "@storybook/svelte";
import { Textarea } from "./index.js";

const meta = {
	title: "Pure/Textarea",
	component: Textarea,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { placeholder: "Enter description..." } };
export const WithValue: Story = { args: { value: "Multi-line\ncontent here" } };
export const Disabled: Story = { args: { placeholder: "Disabled", disabled: true } };
