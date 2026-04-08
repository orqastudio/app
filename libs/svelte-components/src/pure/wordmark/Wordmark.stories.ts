import type { Meta, StoryObj } from "@storybook/svelte";
import Wordmark from "./Wordmark.svelte";

const meta = {
	title: "Pure/Wordmark",
	component: Wordmark,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Studio: Story = { args: { suffix: "Studio" } };
export const DevTools: Story = { args: { suffix: "DevTools" } };
export const Small: Story = { args: { suffix: "Studio", size: "xs" } };
export const Medium: Story = { args: { suffix: "Studio", size: "md" } };
