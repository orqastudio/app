import type { Meta, StoryObj } from "@storybook/svelte";
import SectionHeader from "./SectionHeader.svelte";

const meta = {
	title: "Pure/SectionHeader",
	component: SectionHeader,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Section: Story = { args: { variant: "section" } };
export const Subsection: Story = { args: { variant: "subsection" } };
export const Compact: Story = { args: { variant: "compact" } };
export const MutedBackground: Story = { args: { variant: "section", background: "muted" } };
