<!-- Renders streaming text content with a blinking cursor indicator.
     Uses a pre element to preserve whitespace during active streaming.
     The cursor blinks via a scoped CSS animation — this cannot be expressed
     with Tailwind utility classes alone. -->
<script lang="ts">
	let { content }: { content: string } = $props();
</script>

<!-- pre preserves whitespace and line breaks during streaming output -->
<pre class="streaming-text">{content}<span class="cursor-blink" aria-hidden="true"></span></pre>

<style>
	.streaming-text {
		margin: 0;
		line-height: 1.625;
	}

	.cursor-blink {
		display: inline-block;
		width: 2px;
		height: 1em;
		background-color: currentColor;
		vertical-align: text-bottom;
		margin-left: 1px;
		animation: blink 1s step-start infinite;
	}

	@keyframes blink {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0;
		}
	}
</style>
