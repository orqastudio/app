import type { Meta, StoryObj } from "@storybook/svelte";
import Link from "./Link.svelte";

const meta = {
	title: "Pure/Link",
	component: Link,
	tags: ["autodocs"],
	argTypes: {
		href: { control: "text" },
		variant: {
			control: "select",
			options: ["default", "muted", "destructive"],
		},
		external: { control: "boolean" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: { href: "#", variant: "default" },
};

export const Muted: Story = {
	args: { href: "#", variant: "muted" },
};

export const Destructive: Story = {
	args: { href: "#", variant: "destructive" },
};

export const External: Story = {
	args: { href: "https://example.com", external: true },
};
