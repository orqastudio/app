<!-- EventDrawer — a right-side panel that shows detailed information about a
     selected log event using 4 tabs (Stack, Context, Related, Raw). Tab content
     is passed via snippets so the consumer decides what renders in each tab,
     keeping the drawer generic and reusable. Keyboard navigation: Escape closes,
     ArrowUp navigates to previous event, ArrowDown navigates to next event. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { Panel } from "../panel/index.js";
	import { Stack } from "../layout/index.js";
	import { HStack } from "../layout/index.js";
	import { SectionHeader } from "../section-header/index.js";
	import { Heading } from "../typography/index.js";
	import { Button } from "../button/index.js";
	import { Icon } from "../icon/index.js";
	import { ScrollArea } from "../scroll-area/index.js";
	import { TabsRoot, TabsList, TabsTrigger, TabsContent } from "../tabs/index.js";

	/**
	 * Minimal shape of a log event required by the EventDrawer.
	 * Consumers can pass a more specific type that extends this.
	 */
	export interface LogEvent {
		readonly id: number;
		readonly level: string;
		readonly source: string;
		readonly message: string;
		readonly timestamp: number;
		readonly category: string;
	}

	/**
	 * Props for EventDrawer. Tab content is provided via snippets
	 * so the consumer fully controls what is rendered in each tab.
	 */
	export interface EventDrawerProps {
		/** Whether the drawer is visible. */
		open: boolean;
		/** The selected log event, or null when nothing is selected. */
		event: LogEvent | null;
		/** Which tab is active. Defaults to "stack". */
		activeTab?: "stack" | "context" | "related" | "raw";
		/** Fired when the drawer should close (Escape or close button). */
		onclose?: () => void;
		/** Fired when the active tab changes. */
		ontabchange?: (tab: string) => void;
		/** Fired when the user navigates to the next event (ArrowDown or button). */
		onnext?: () => void;
		/** Fired when the user navigates to the previous event (ArrowUp or button). */
		onprev?: () => void;
		/** Content for the Stack tab. */
		stackContent?: Snippet;
		/** Content for the Context tab. */
		contextContent?: Snippet;
		/** Content for the Related tab. */
		relatedContent?: Snippet;
		/** Content for the Raw tab. */
		rawContent?: Snippet;
	}

	let {
		open,
		activeTab = "stack",
		onclose,
		ontabchange,
		onnext,
		onprev,
		stackContent,
		contextContent,
		relatedContent,
		rawContent,
	}: EventDrawerProps = $props();

	/**
	 * Handles keyboard events on the drawer container.
	 * Escape closes the drawer; ArrowUp navigates to the previous event;
	 * ArrowDown navigates to the next event.
	 * @param e - The keyboard event.
	 */
	function handleKeydown(e: KeyboardEvent): void {
		if (e.key === "Escape") {
			e.preventDefault();
			onclose?.();
		} else if (e.key === "ArrowUp") {
			e.preventDefault();
			onprev?.();
		} else if (e.key === "ArrowDown") {
			e.preventDefault();
			onnext?.();
		}
	}

	/**
	 * Forwards the tab change from TabsRoot to the consumer via ontabchange.
	 * @param value - The new active tab value.
	 */
	function handleTabChange(value: string): void {
		ontabchange?.(value);
	}
</script>

<svelte:window onkeydown={open ? handleKeydown : undefined} />

{#if open}
	<Panel padding="none" border="all" flex={0} width="auto">
		<Stack gap={0} height="full">
			<SectionHeader>
				{#snippet start()}
					<Heading level={6}>Event Detail</Heading>
				{/snippet}
				{#snippet end()}
					<HStack gap={1}>
						<Button variant="ghost" size="icon-sm" onclick={onprev} aria-label="Previous event">
							<Icon name="chevron-up" size="sm" />
						</Button>
						<Button variant="ghost" size="icon-sm" onclick={onnext} aria-label="Next event">
							<Icon name="chevron-down" size="sm" />
						</Button>
						<Button variant="ghost" size="icon-sm" onclick={onclose} aria-label="Close drawer">
							<Icon name="x" size="sm" />
						</Button>
					</HStack>
				{/snippet}
			</SectionHeader>
			<TabsRoot value={activeTab} onValueChange={handleTabChange}>
				<TabsList>
					<TabsTrigger value="stack">Stack</TabsTrigger>
					<TabsTrigger value="context">Context</TabsTrigger>
					<TabsTrigger value="related">Related</TabsTrigger>
					<TabsTrigger value="raw">Raw</TabsTrigger>
				</TabsList>
				<ScrollArea full>
					<TabsContent value="stack">
						{@render stackContent?.()}
					</TabsContent>
					<TabsContent value="context">
						{@render contextContent?.()}
					</TabsContent>
					<TabsContent value="related">
						{@render relatedContent?.()}
					</TabsContent>
					<TabsContent value="raw">
						{@render rawContent?.()}
					</TabsContent>
				</ScrollArea>
			</TabsRoot>
		</Stack>
	</Panel>
{/if}
