import type { Meta, StoryObj } from "@storybook/svelte";
import HighlightWrapper from "./HighlightWrapper.svelte";

const meta = {
	title: "Pure/HighlightWrapper",
	component: HighlightWrapper,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

// HighlightWrapper wraps svelte-highlight output — stories verify the scroll
// container and style-override classes render without visual regressions.
export const Default: Story = { args: {} };
