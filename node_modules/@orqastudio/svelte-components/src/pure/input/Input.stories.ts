import type { Meta, StoryObj } from "@storybook/svelte";
import { Input } from "./index.js";

const meta = {
	title: "Pure/Input",
	component: Input,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { placeholder: "Enter text..." } };
export const WithValue: Story = { args: { value: "Hello world" } };
export const Disabled: Story = { args: { placeholder: "Disabled", disabled: true } };
