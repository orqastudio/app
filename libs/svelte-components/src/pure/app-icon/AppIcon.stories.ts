import type { Meta, StoryObj } from "@storybook/svelte";
import AppIcon from "./AppIcon.svelte";

const meta = {
	title: "Pure/AppIcon",
	component: AppIcon,
	tags: ["autodocs"],
} satisfies Meta<typeof AppIcon>;

export default meta;
type Story = StoryObj<typeof meta>;

// Use a data URI placeholder for story previews.
const placeholder =
	"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 40 40'%3E%3Crect width='40' height='40' rx='6' fill='%236366f1'/%3E%3C/svg%3E";

export const ExtraSmall: Story = {
	args: { src: placeholder, alt: "Brand mark", size: "xs" },
};

export const Small: Story = {
	args: { src: placeholder, alt: "App logo", size: "sm" },
};

export const Medium: Story = {
	args: { src: placeholder, alt: "App logo", size: "md" },
};

export const RoundedObjectContain: Story = {
	args: { src: placeholder, alt: "Project icon", size: "sm", rounded: true, objectContain: true },
};
