<!-- API/Models Page -->
<script lang="ts">
	import { enhance } from '$app/forms';
	import { LlmSdk, llmState } from '../state.svelte';

	// const endpointSdkOptions: LlmSdk[] = ['OpenAI', 'Anthropic', 'OpenAI Compatible'];
	let emptyConfig: ApiConfig = {
		id: '',
		name: '',
		endpoint_sdk: LlmSdk.OpenAI,
		base_url: ''
	};
	// let apiConfigs: ApiConfig[] = $state([])
	let editingConfig: ApiConfig = $state(emptyConfig);
	let apiEditModal: HTMLDialogElement;
</script>

<div class="custom_border ml-4 p-4">
	<div class="my-4 flex items-center justify-between gap-4 border-b px-2 last:border-b-0">
		<p class="info_paragraph">
			You can add <strong>API keys</strong> for various services like
			<strong>OpenAI, Claude, Groq(Llama) and other AI providers</strong>. Provide the
			<strong>base URL</strong>
			for the API and <strong>specify the models</strong> available for use. This will allow you to easily
			integrate and manage different AI models in your application.
		</p>
		<button
			type="button"
			class="btn_emerald h-fit min-w-fit"
			onclick={() => apiEditModal.showModal()}
		>
			Add New API / Models
		</button>
	</div>
	{#each llmState.settings.apiConfigs as config}
		<div class="flex items-center justify-between border-b p-2 last:border-b-0">
			<!-- last:border-b-0 to remove border from last element -->
			<div class="flex-1">
				<h3 class="text-lg font-semibold">{config.name}</h3>
				<p class="text-sm text-gray-500">Base URL: {config.base_url}</p>
				<h4 class="text-md font-semibold">Models:</h4>
				<ul>
					{#each config.models ?? [] as model}
						<li class="text-sm text-gray-500">{model}</li>
					{/each}
				</ul>
			</div>
			<div class="flex items-center gap-1">
				<button
					type="button"
					class="btn_emerald"
					onclick={() => {
						editingConfig = config;
						apiEditModal.showModal();
					}}>Edit</button
				>
				<!-- Form for deletion -->
				<form method="POST" action="?/deleteApiConfig" use:enhance>
					<input type="hidden" name="id" value={config.id} />
					<button class="btn_red">Delete</button>
				</form>
			</div>
		</div>
	{/each}
</div>

<dialog bind:this={apiEditModal} class="modal" onclose={() => (editingConfig = emptyConfig)}>
	<div class="modal-box">
		<form method="POST" action="?/saveApiConfig" use:enhance onsubmit={() => apiEditModal.close()}>
			<!-- Hidden Inputs to pass additional data -->
			<input type="hidden" name="id" value={editingConfig.id} />
			<input type="hidden" name="apiConfigsLength" value={llmState.settings.apiConfigs.length} />

			<!-- Endpoint SDK Dropdown  -->
			<label class="flex items-center justify-end gap-2">
				<div class="label">
					<span class="label-text">Endpoint Compatibility SDK:</span>
				</div>
				<select bind:value={editingConfig.endpoint_sdk} class="select select-bordered select-sm">
					{#each Object.values(LlmSdk) as sdk}
						<option value={sdk}
							>{LlmSdk.OpenAICompatible === sdk ? 'OpenAI Compatible' : sdk}</option
						>
					{/each}
				</select>
				<input type="hidden" name="endpoint-sdk" value={editingConfig.endpoint_sdk} />
			</label>
			<!-- Name Input -->
			<div class="form-control">
				<label class="label" for="name">
					<span class="label-text">Name</span>
				</label>
				<!-- svelte-ignore a11y_autofocus -->
				<input
					name="name"
					type="text"
					class="input input-bordered w-full"
					required
					autofocus
					bind:value={editingConfig.name}
				/>
			</div>

			<!-- API Key Input -->
			<div class="form-control">
				<label class="label" for="api-key">
					<span class="label-text">API Key (OpenAI, Claude, etc.)</span>
				</label>
				<input
					name="api-key"
					type="text"
					class="input input-bordered w-full"
					placeholder="Leave Empty if already saved in config"
					bind:value={editingConfig.secret_key}
				/>
			</div>

			<!-- Base URL Input -->
			{#if editingConfig.endpoint_sdk === LlmSdk.OpenAICompatible}
				<div class="form-control">
					<label class="label" for="base-url">
						<span class="label-text">Base URL</span>
					</label>
					<input
						name="base-url"
						type="text"
						class="input input-bordered w-full"
						required
						bind:value={editingConfig.base_url}
					/>
				</div>
			{/if}

			<!-- Models Input -->
			<div class="form-control mt-4">
				<!-- Divider with 'Models' -->
				<div class="relative my-4">
					<hr class="border-t border-gray-500" />
					<span
						class="absolute left-1/2 -translate-x-1/2 -translate-y-1/2 transform bg-base-100 px-2 text-lg font-semibold text-white"
					>
						Models
					</span>
				</div>
				<ul class="form-control gap-1">
					{#each editingConfig.models ?? [] as _, index}
						<li class="flex flex-row">
							<label class="label min-w-fit" for={`models-${index}`}>
								<span class="label-text">Model Name</span>
							</label>
							<input
								name={`models-${index}`}
								type="text"
								class="input input-bordered w-full"
								bind:value={editingConfig.models![index]}
							/>
						</li>
					{/each}
				</ul>
				<button
					type="button"
					class="btn_emerald my-1 w-fit"
					onclick={() => editingConfig.models?.push('')}>Add Model</button
				>
			</div>

			<!-- Save Button -->
			<div class="modal-action flex gap-4">
				<button type="button" class="link-hover" onclick={() => apiEditModal.close()}>Cancel</button
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
	.info_paragraph strong {
		@apply text-info text-opacity-95;
	}
</style>
