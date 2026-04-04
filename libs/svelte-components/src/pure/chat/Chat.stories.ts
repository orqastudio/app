import type { Meta, StoryObj } from "@storybook/svelte";
import ChatBubble from "./ChatBubble.svelte";
import ChatInput from "./ChatInput.svelte";
import StreamingDots from "./StreamingDots.svelte";
import ChatContainer from "./ChatContainer.svelte";

// ── ChatBubble ────────────────────────────────────────────────────────────────

export const BubbleMeta = {
	title: "Pure/Chat/ChatBubble",
	component: ChatBubble,
	tags: ["autodocs"],
	argTypes: {
		role: {
			control: "select",
			options: ["user", "assistant", "system"],
		},
	},
} satisfies Meta;

export default BubbleMeta;
type BubbleStory = StoryObj<typeof BubbleMeta>;

export const UserBubble: BubbleStory = {
	args: { role: "user" },
};

export const AssistantBubble: BubbleStory = {
	args: { role: "assistant" },
};

export const SystemBubble: BubbleStory = {
	args: { role: "system" },
};

// ── ChatInput ─────────────────────────────────────────────────────────────────

export const InputMeta = {
	title: "Pure/Chat/ChatInput",
	component: ChatInput,
	tags: ["autodocs"],
	argTypes: {
		isStreaming: { control: "boolean" },
		disabled: { control: "boolean" },
	},
} satisfies Meta;

type InputStory = StoryObj<typeof InputMeta>;

export const InputDefault: InputStory = {
	args: {
		placeholder: "Type a message...",
		isStreaming: false,
		disabled: false,
		onsubmit: (content: string) => console.log("send:", content),
	},
};

export const InputStreaming: InputStory = {
	args: {
		placeholder: "Type a message...",
		isStreaming: true,
		onsubmit: (content: string) => console.log("send:", content),
		onstop: () => console.log("stop"),
	},
};

export const InputDisabled: InputStory = {
	args: {
		placeholder: "Type a message...",
		disabled: true,
		onsubmit: (content: string) => console.log("send:", content),
	},
};

// ── StreamingDots ─────────────────────────────────────────────────────────────

export const DotsMeta = {
	title: "Pure/Chat/StreamingDots",
	component: StreamingDots,
	tags: ["autodocs"],
} satisfies Meta;

type DotsStory = StoryObj<typeof DotsMeta>;

export const DotsDefault: DotsStory = {
	args: {},
};

// ── ChatContainer ─────────────────────────────────────────────────────────────

export const ContainerMeta = {
	title: "Pure/Chat/ChatContainer",
	component: ChatContainer,
	tags: ["autodocs"],
} satisfies Meta;

type ContainerStory = StoryObj<typeof ContainerMeta>;

export const ContainerDefault: ContainerStory = {
	args: {},
};
