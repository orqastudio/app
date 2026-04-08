import type { Meta, StoryObj } from "@storybook/svelte";
import SurfaceBox from "./SurfaceBox.svelte";

const meta: Meta<typeof SurfaceBox> = {
	title: "Pure/SurfaceBox",
	component: SurfaceBox,
	tags: ["autodocs"],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = { args: {} };
export const Centered: Story = { args: { center: true, style: "width: 200px; height: 80px;" } };
