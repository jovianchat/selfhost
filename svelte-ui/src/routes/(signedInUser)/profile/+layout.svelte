<script lang="ts">
	import { page } from '$app/stores';
	let { children } = $props();
	const links = [{ name: 'Change Password', href: '/profile/password' }];
</script>

<div class="flex min-h-screen flex-col">
	<div class="mx-auto my-32 flex w-5/6 flex-1 flex-row">
		<ul class="my-10 flex w-3/12 flex-col gap-2">
			{#each links as link}
				<li class="truncate text-center">
					<a
						href={link.href}
						class="btn btn-ghost w-full justify-center text-lg"
						class:active={link.href === $page.url.pathname}>{link.name}</a
					>
				</li>
			{/each}
		</ul>
		<div class="flex-1">
			{@render children()}
		</div>
	</div>
</div>

<style lang="postcss">
	.active {
		@apply bg-base-300 bg-opacity-65 text-accent;
	}
</style>
