<script lang="ts">
	import ChatTopbar from './Topbar.svelte';
	import TextArea from './TextArea.svelte';
	import FavModels from '$/components/FavModels.svelte';

	const { children } = $props();
</script>

<div class="mx-auto flex min-h-screen flex-col justify-between px-6">
	<div class="sticky top-0 bg-base-100">
		<div class="syncSidebarTopbar_Alignment relative flex flex-nowrap items-center justify-between">
			<ChatTopbar />
		</div>
	</div>

	<div class="flex flex-1 justify-center">
		<div class="flex flex-1 flex-col">
			<FavModels />
		</div>

		<div class="content_chattextarea_width flex flex-col justify-between">
			<div class="flex flex-1 flex-col">
				{@render children()}
			</div>
			<div class="sticky bottom-0 bg-base-100">
				<div class="mt-4">
					<TextArea />
				</div>
			</div>
		</div>

		<div class="flex-1"></div>
	</div>
</div>

<style lang="postcss">
	.content_chattextarea_width {
		@apply mx-auto w-7/12 max-w-screen-lg;
	}
</style>
