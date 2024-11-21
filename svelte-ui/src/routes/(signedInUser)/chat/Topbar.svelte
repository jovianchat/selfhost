<script lang="ts">
	import { page } from '$app/stores';
	import { tick } from 'svelte';
	import TablerLayoutSidebarLeftExpandFilled from '~icons/tabler/layout-sidebar-left-expand-filled';
	import GridiconsChat from '~icons/gridicons/chat';
	import MdiRename from '~icons/mdi/rename';
	import WeuiDeleteOnOutlined from '~icons/weui/delete-on-outlined';
	import MdiStarOutline from '~icons/mdi/star-outline';
	import MdiStar from '~icons/mdi/star';

	import { sidebarState } from '$/components/layout/sidebar.svelte';
	import { actions } from './topbarActions';
	import { chatState } from './[chatId]/state.svelte';
	import LlmControls from '../llmSettings/LlmControls.svelte';

	let chatId = $derived($page.params.chatId);
	let chatTitle = $derived($page.data?.chat?.details.title);
	let chatStarred = $derived($page.data?.chat?.details.starred);
	$effect(() => {
		chatState.starred = chatStarred;
		chatState.title = chatTitle;
	});

	let isRenaming = $state(false);
	// svelte-ignore non_reactive_update
	let renameTextField: HTMLInputElement;
</script>

<div class="flex items-center gap-6">
	{#if !sidebarState.isOpen}
		<div class="tooltip tooltip-right mt-2" data-tip="Open Sidebar">
			<button
				class="btn btn-square btn-ghost btn-sm"
				onclick={() => sidebarState.toggle(localStorage)}
			>
				<TablerLayoutSidebarLeftExpandFilled class="h-9 w-9" />
			</button>
		</div>
		<a href="/" class="text-xl font-semibold">Jovian Chat</a>
	{/if}
</div>
<div class="absolute left-1/2 z-10 flex -translate-x-1/2 items-center gap-2">
	{#if chatId !== 'new' && chatId !== undefined}
		<GridiconsChat class="h-5 w-5" />
		{#if !isRenaming}
			<p>{chatState.title}</p>
		{:else}
			<form
				onsubmit={(e) => {
					e.preventDefault();
					isRenaming = false;
					actions.renameChat(chatId, chatState.title);
				}}
			>
				<input
					type="text"
					bind:this={renameTextField}
					bind:value={chatState.title}
					onblur={() => (isRenaming = false)}
					class="input input-sm input-accent w-full"
				/>
			</form>
		{/if}
	{/if}
</div>
<div class="flex gap-6">
	{#if chatId !== 'new' && chatId !== undefined}
		<div class="tooltip tooltip-bottom" data-tip="Rename Chat">
			<button
				type="button"
				onclick={() => {
					isRenaming = true;
					tick().then(() => {
						renameTextField?.focus();
					});
				}}
				class="btn btn-square btn-ghost btn-sm"
			>
				<MdiRename class="h-6 w-6" />
			</button>
		</div>
		<div class="tooltip tooltip-bottom" data-tip="Delete Chat">
			<button
				type="button"
				onclick={() => {
					actions.deleteChat(chatId);
				}}
				class="btn btn-square btn-ghost btn-sm"
			>
				<WeuiDeleteOnOutlined class="h-6 w-6" />
			</button>
		</div>
		<div class="tooltip tooltip-bottom" data-tip="Star Chat">
			<button
				type="button"
				onclick={() => {
					actions.starChat(chatId);
				}}
				class="btn btn-square btn-ghost btn-sm"
			>
				{#if chatState.starred}
					<MdiStar class="h-6 w-6 text-yellow-500" />
				{:else}
					<MdiStarOutline class="h-6 w-6" />
				{/if}
			</button>
		</div>
	{/if}
	<LlmControls />
</div>
