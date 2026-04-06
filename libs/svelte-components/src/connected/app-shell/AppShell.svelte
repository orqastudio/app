<script lang="ts">
	import type { Snippet } from "svelte";
	import {
		ResizablePaneGroup,
		ResizableHandle,
		ResizablePane,
	} from "../../pure/resizable/index.js";

	let {
		toolbar,
		activityBar,
		navPanel,
		mainContent,
		chatPanel,
		statusBar,
		overlays,
		showNavPanel = true,
		showChatPanel = true,
		chatPanelSize = 30,
		mainPanelSize = 70,
	}: {
		toolbar?: Snippet;
		activityBar?: Snippet;
		navPanel?: Snippet;
		mainContent: Snippet;
		chatPanel?: Snippet;
		statusBar?: Snippet;
		overlays?: Snippet;
		showNavPanel?: boolean;
		showChatPanel?: boolean;
		chatPanelSize?: number;
		mainPanelSize?: number;
	} = $props();
</script>

<div class="bg-background text-foreground flex h-screen flex-col">
	{#if toolbar}
		{@render toolbar()}
	{/if}

	<div class="flex flex-1 overflow-hidden">
		{#if activityBar}
			{@render activityBar()}
		{/if}

		{#if showNavPanel && navPanel}
			{@render navPanel()}
		{/if}

		{#if showChatPanel && chatPanel}
			<div class="min-w-0 flex-1">
				<ResizablePaneGroup direction="horizontal">
					<ResizablePane defaultSize={mainPanelSize} minSize={30}>
						<div class="h-full overflow-hidden">
							{@render mainContent()}
						</div>
					</ResizablePane>
					<ResizableHandle />
					<ResizablePane defaultSize={chatPanelSize} minSize={20}>
						<div class="bg-chat flex h-full flex-col">
							{@render chatPanel()}
						</div>
					</ResizablePane>
				</ResizablePaneGroup>
			</div>
		{:else}
			<div class="min-w-0 flex-1 overflow-hidden">
				{@render mainContent()}
			</div>
		{/if}
	</div>

	{#if statusBar}
		{@render statusBar()}
	{/if}

	{#if overlays}
		{@render overlays()}
	{/if}
</div>
