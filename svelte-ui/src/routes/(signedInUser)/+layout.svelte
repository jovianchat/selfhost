<script lang="ts">
	import Sidebar from '$/components/layout/Sidebar.svelte';
	import { sidebarState } from '$/components/layout/sidebar.svelte';
	import { slide } from 'svelte/transition';
	import { page } from '$app/stores';
	import { llmState } from './llmSettings/state.svelte';
	import { onMount } from 'svelte';

	let { data, children } = $props();
	let { chats, llmSettings } = $derived(data);
	$effect(() => {
		llmSettings;
		llmState.settings = llmSettings;
	});
	onMount(() => {
		const storedId = localStorage.getItem('activeFavId');
		llmState.setActiveFavById(storedId ? Number(storedId) : llmState.defaultFavModels[0]);
	});
	let isChatPage = $derived($page.url.pathname.startsWith('/chat'));
</script>

<main>
	<div class="flex">
		{#if sidebarState.isOpen || !isChatPage}
			<div
				class="page_height sticky top-0 w-[20%] max-w-72 bg-base-300"
				transition:slide={{ delay: 20, duration: 800, axis: 'x' }}
			>
				<Sidebar {chats} />
			</div>
		{/if}
		<div class="flex-1">
			<!-- Flex 1 means full width left from flex -->
			{@render children()}
		</div>
	</div>
</main>

<style lang="postcss">
	:global(.syncSidebarTopbar_Alignment) {
		@apply my-1 h-12;
	}
	:global(.page_height) {
		@apply h-screen;
	}
</style>
