import type { Meta, StoryObj } from "@storybook/svelte";
import Dot from "./Dot.svelte";

const meta = {
	title: "Pure/Dot",
	component: Dot,
	tags: ["autodocs"],
	argTypes: {
		size: {
			control: "select",
			options: ["xs", "sm", "md"],
		},
		color: {
			control: "select",
			options: ["primary", "success", "warning", "destructive", "muted", "info"],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { size: "sm", color: "muted" } };
export const XsPrimary: Story = { args: { size: "xs", color: "primary" } };
export const XsSuccess: Story = { args: { size: "xs", color: "success" } };
export const XsWarning: Story = { args: { size: "xs", color: "warning" } };
export const XsDestructive: Story = { args: { size: "xs", color: "destructive" } };
export const XsMuted: Story = { args: { size: "xs", color: "muted" } };
export const XsInfo: Story = { args: { size: "xs", color: "info" } };
export const SmPrimary: Story = { args: { size: "sm", color: "primary" } };
export const SmSuccess: Story = { args: { size: "sm", color: "success" } };
export const SmWarning: Story = { args: { size: "sm", color: "warning" } };
export const SmDestructive: Story = { args: { size: "sm", color: "destructive" } };
export const SmInfo: Story = { args: { size: "sm", color: "info" } };
export const MdPrimary: Story = { args: { size: "md", color: "primary" } };
export const MdSuccess: Story = { args: { size: "md", color: "success" } };
export const MdWarning: Story = { args: { size: "md", color: "warning" } };
export const MdDestructive: Story = { args: { size: "md", color: "destructive" } };
export const MdMuted: Story = { args: { size: "md", color: "muted" } };
export const MdInfo: Story = { args: { size: "md", color: "info" } };
