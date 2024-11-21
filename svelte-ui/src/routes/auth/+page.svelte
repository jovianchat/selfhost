<script lang="ts">
	import { enhance } from '$app/forms';
	let password = $state('');
</script>

<div class="flex h-screen flex-col">
	<form method="POST" action="?/login" use:enhance class="h-3/12 m-auto w-3/12 p-4">
		<div class="form-control">
			<label class="label" for="password">
				<span class="label-text">Password</span>
			</label>
			<input
				name="password"
				type="password"
				class="input input-bordered w-full"
				required
				bind:value={password}
			/>
		</div>
		<button type="submit" class="btn_emerald btn mt-3 w-full">Sign In</button>
	</form>
</div>
