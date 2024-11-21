<script lang="ts">
	import { afterNavigate, beforeNavigate } from '$app/navigation';
	import { marked } from 'marked';
	import { chatState } from './state.svelte';
	import { tick } from 'svelte';
	import { esClose_SaveDb } from './chatResponse';

	const { data } = $props();
	const { chat } = $derived(data);

	beforeNavigate(({ from, to, cancel }) => {
		if (from?.url.pathname === to?.url.pathname) {
			cancel();
		} else {
			const chatId = from?.url.pathname.split('/').pop();
			esClose_SaveDb(chatId);
			chatState.emptyQR();
		}
	});
	afterNavigate(() => scrollToBottom({ instant: true }));

	// Delay settings
	let scrollTimeout: NodeJS.Timeout | null = null;
	const scrollToBottom = ({ instant = false } = {}) => {
		// Clear the previous timeout if exists to reduce scroll events
		if (scrollTimeout) clearTimeout(scrollTimeout);

		// Set a timeout to delay scroll calculation by 50 ms
		scrollTimeout = setTimeout(() => {
			tick().then(() => {
				window.scrollTo({
					top: document.body.scrollHeight,
					behavior: instant ? 'instant' : 'smooth'
				});
			});
		}, 50);
	};
	// svelte:window eventListeners
	let userScrolledUp = $state(false);
	function handleWheel(event: WheelEvent) {
		if (event.deltaY < 0) {
			userScrolledUp = true;
		}
	}
	function handleScrollBottom() {
		const isAtBottom = window.innerHeight + window.scrollY >= document.body.scrollHeight - 5;
		if (isAtBottom) {
			userScrolledUp = false; // Reset the flag when user reaches bottom
		}
	}

	// Scroll on dependency change
	$effect.pre(() => {
		if (chatState.qr.length > 0) chatState.qr[chatState.qr.length - 1].assistant_response;

		// let scrollUp = window.scrollY + window.innerHeight < document.body.scrollHeight - 1;
		if (chatState.qr.length > 0 && !userScrolledUp) {
			scrollToBottom();
		}
	});
	$effect(() => {
		chatState.qr.length;
		scrollToBottom();
	});
</script>

<svelte:window onscroll={handleScrollBottom} onwheel={handleWheel} />

<div class="flex flex-col gap-6">
	{#each chat.messages as { user_query, assistant_response }}
		<div class="query_bg custom_border prose prose-sm ml-auto w-fit max-w-[92%] overflow-auto">
			<div class="prose-cyan whitespace-pre px-4 py-2 shadow-sm">
				{user_query}
			</div>
		</div>
		<div class="prose min-w-full">
			<div class="prose-cyan rounded-lg bg-base-300 px-4 py-2 shadow-md">
				{@html marked.parse(assistant_response)}
			</div>
		</div>
	{/each}
	{#each chatState.qr as { user_query, assistant_response }}
		<div class="query_bg custom_border prose prose-sm ml-auto w-fit max-w-[92%] overflow-auto">
			<div class="prose-cyan whitespace-pre px-4 py-2 shadow-sm">
				{user_query}
			</div>
		</div>
		<div class="prose min-w-full">
			<div class="prose-cyan rounded-lg bg-base-300 px-4 py-2 shadow-md">
				{@html marked.parse(assistant_response)}
			</div>
		</div>
	{/each}
</div>

<style lang="postcss">
	.custom_border {
		border-width: 1px;
		border-color: var(--fallback-bc, oklch(var(--bc) / 0.2));
	}
	.query_bg {
		background-color: #30302d;
		@apply rounded-badge;
	}
</style>
