<script lang="ts">
	import MdiKeyboardArrowDown from '~icons/mdi/keyboard-arrow-down';
	let user = {
		email: 'admin'
	};
</script>

{#if user}
	<div class="dropdown dropdown-end dropdown-top mb-1 mt-auto">
		<div tabindex="0" role="button" class="btn btn-ghost flex flex-nowrap items-center gap-2 p-2">
			<div class="flex min-w-0 items-center">
				<span class="block truncate text-lg">{user.email}</span>
				<MdiKeyboardArrowDown class="h-7 w-7" />
			</div>
		</div>
		<ul class="menu dropdown-content menu-lg z-[1] mb-1 w-full rounded-box bg-base-200 p-2 shadow">
			<li>
				<a href="/profile/password"> Profile </a>
			</li>
			<form method="POST" action="/auth?/logout">
				<li>
					<button type="submit">Logout</button>
				</li>
			</form>
		</ul>
	</div>
{/if}
