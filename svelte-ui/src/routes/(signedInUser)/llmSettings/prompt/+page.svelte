<!-- Prompt Engineering Page -->
<script lang="ts">
	import { enhance } from '$app/forms';
	import { onMount } from 'svelte';
	import { llmState } from '../state.svelte';

	let emptyconfig: PromptConfig = { name: '', max_tokens: 0, temperature: 0, system_prompt: '' };
	let editingConfig: PromptConfig = $state(emptyconfig);
	let promptEditModal: HTMLDialogElement;

	onMount(() => {
		promptEditModal.addEventListener('close', () => {
			editingConfig = emptyconfig;
		});
	});
</script>

<div class="custom_border ml-4 p-4">
	<div class="mb-4 flex justify-end px-2">
		<button type="button" class="btn_emerald" onclick={() => promptEditModal.showModal()}>
			Add New Prompt Configuration
		</button>
	</div>
	{#each llmState.settings.promptConfigs as config}
		<div class="flex items-center justify-between border-b p-2 last:border-b-0">
			<!-- last:border-b-0 to remove border from last element -->
			<div class="flex-1">
				<h3 class="text-lg font-semibold">{config.name}</h3>
				<p class="text-sm text-gray-500">Max Tokens: {config.max_tokens}</p>
				<p class="text-sm text-gray-500">Temperature: {config.temperature}</p>
				<p class="text-sm text-gray-500">System Prompt: {config.system_prompt}</p>
			</div>
			<div class="flex items-center gap-1">
				<button
					type="button"
					class="btn_emerald"
					onclick={() => {
						editingConfig = config;
						promptEditModal.showModal();
					}}>Edit</button
				>
				<!-- Form for deletion -->
				<form method="POST" action="?/deletePromptConfig" use:enhance>
					<input type="hidden" name="id" value={config.id} />
					<button class="btn_red">Delete</button>
				</form>
			</div>
		</div>
	{/each}
</div>

<dialog bind:this={promptEditModal} class="modal">
	<div class="modal-box">
		<form
			method="POST"
			action="?/savePromptConfig"
			use:enhance
			onsubmit={() => promptEditModal.close()}
		>
			<!-- Hidden Inputs to pass additional data -->
			<input type="hidden" name="id" value={editingConfig.id} />
			<input
				type="hidden"
				name="promptConfigsLength"
				value={llmState.settings.promptConfigs.length}
			/>

			<!-- Name Input -->
			<div class="form-control">
				<label class="label" for="name">
					<span class="label-text">Name</span>
				</label>
				<input
					name="name"
					type="text"
					class="input input-bordered w-full"
					required
					bind:value={editingConfig.name}
				/>
			</div>

			<!-- Max Tokens Input -->
			<div class="form-control">
				<label class="label" for="max-tokens">
					<span class="label-text">Max Tokens</span>
				</label>
				<input
					name="max-tokens"
					type="number"
					min="512"
					class="input input-bordered w-full"
					bind:value={editingConfig.max_tokens}
				/>
			</div>

			<!-- Temperature Input -->
			<div class="form-control">
				<label class="label" for="temperature">
					<span class="label-text">Temperature</span>
				</label>
				<input
					name="temperature"
					type="number"
					step="0.01"
					min="0"
					max="1"
					class="input input-bordered w-full"
					bind:value={editingConfig.temperature}
				/>
			</div>

			<!-- System Prompt Input -->
			<div class="form-control">
				<label class="label" for="system-prompt">
					<span class="label-text">System Prompt</span>
				</label>
				<input
					name="system-prompt"
					type="text"
					class="input input-bordered w-full"
					required
					bind:value={editingConfig.system_prompt}
				/>
			</div>

			<!-- Save Button -->
			<div class="modal-action flex gap-4">
				<button type="button" class="link-hover" onclick={() => promptEditModal.close()}
					>Cancel</button
				>
				<button type="submit" class="btn_emerald">Save</button>
			</div>
		</form>
	</div>
</dialog>

<style lang="postcss">
	.custom_border {
		border-width: 2px;
		border-color: var(--fallback-bc, oklch(var(--bc) / 0.2));
		@apply rounded-lg;
	}

	/* Hide input number arrows */
	/* Chrome, Safari, Edge, Opera */
	input::-webkit-outer-spin-button,
	input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}
	/* Firefox */
	input[type='number'] {
		-moz-appearance: textfield;
	}
</style>
