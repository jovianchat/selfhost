<script lang="ts">
	import { enhance } from '$app/forms';
</script>

<div class="ml-4 p-4">
	<form method="POST" use:enhance class="flex flex-col gap-4">
		<div class="form-control">
			<label class="label" for="new-password">
				<span class="label-text">New Password</span>
			</label>
			<input
				name="new-password"
				type="password"
				placeholder="New Password"
				class="input input-bordered w-full"
				required
			/>
		</div>
		<div class="form-control">
			<label class="label" for="confirm-password">
				<span class="label-text">Confirm Password</span>
			</label>
			<input
				name="confirm-password"
				type="password"
				placeholder="Confirm Password"
				class="input input-bordered w-full"
				required
			/>
		</div>
		<button type="submit" class="btn_emerald btn">Change Password</button>
	</form>
</div>
