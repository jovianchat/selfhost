<!-- LLM Controls Component -->
<script lang="ts">
	import { enhance } from '$app/forms';
	import MdiClose from '~icons/mdi/close';
	import FluentSettingsChat16Filled from '~icons/fluent/settings-chat-16-filled';
	import { llmState } from './state.svelte';
	// Dialog Element
	let llmControlsModal: HTMLDialogElement;

	// Form fields default values and bind values
	let selectedApi: ApiConfig | undefined = $state();
	let selectedModel: string | undefined = $state();
	$effect(() => {
		// Needed when selected Api is changed from dropdown
		selectedModel = selectedApi?.models![0] ?? '';
	});
	let selectedPromptConfig: PromptConfig | undefined = $state();

	// Local activeFav for favModels Buttons inside the dialog
	let activeFavId: number = $state(llmState.defaultFavModels[0]);
</script>

<div class="tooltip tooltip-left text-accent" data-tip="LLM Controls">
	<button
		type="button"
		class="btn btn-square btn-ghost btn-sm"
		onclick={() => {
			selectedApi = llmState.settings.apiConfigs[0] ?? '';
			// selectedModel = selectedApi.models![0] ?? ''; //Not needed with $effect
			selectedPromptConfig = llmState.settings.promptConfigs[0] ?? '';
			llmControlsModal!.showModal();
		}}
	>
		<FluentSettingsChat16Filled class="h-6 w-6" />
	</button>
</div>
<dialog bind:this={llmControlsModal} class="modal">
	<div class="modal-box">
		<div class="flex items-center justify-between">
			<h3 class="text-lg font-bold">LLM Controls</h3>
			<form method="dialog">
				<button class="btn btn-square btn-ghost btn-sm">
					<MdiClose class="h-6 w-6" />
				</button>
			</form>
		</div>
		<div class="divider my-0"></div>
		<form
			method="POST"
			action="/llmSettings/prompt?/saveFavModel"
			use:enhance={() => {
				return async ({ update }) => {
					await update();
					llmState.setActiveFavById(activeFavId);
					activeFavId = llmState.defaultFavModels[0];
				};
			}}
			onsubmit={() => llmControlsModal.close()}
		>
			<div class="flex flex-col gap-4">
				<!-- API Selector -->
				<label class="form-control w-full max-w-xs">
					<div class="label">
						<span class="label-text">Select API Model:</span>
					</div>
					<select bind:value={selectedApi} class="select select-bordered">
						{#each llmState.settings.apiConfigs as api}
							<option value={api}>{api.name}</option>
						{/each}
					</select>
					<input type="hidden" name="selectedApiId" value={selectedApi?.id} />
				</label>
				<!-- Model Selector for corresponding API -->
				<label class="form-control w-full max-w-xs">
					<div class="label">
						<span class="label-text">Select Model to use available for Selected API:</span>
					</div>
					<select bind:value={selectedModel} class="select select-bordered">
						{#each selectedApi?.models ?? [] as model}
							<option value={model}>{model}</option>
						{/each}
					</select>
					<input type="hidden" name="selectedModel" value={selectedModel} />
				</label>

				<!-- Prompt Selector -->
				<label class="form-control w-full max-w-xs">
					<div class="label">
						<span class="label-text">Select Prompt Engineered Config:</span>
					</div>
					<select bind:value={selectedPromptConfig} class="select select-bordered">
						{#each llmState.settings.promptConfigs as config}
							<option value={config}>{config.name}</option>
						{/each}
					</select>
					<input type="hidden" name="selectedPromptId" value={selectedPromptConfig?.id} />
					<div class="label">
						<div class="label-text">
							{#if selectedPromptConfig}
								<p class="text-sm text-gray-500">
									Max Tokens: {selectedPromptConfig.max_tokens}
								</p>
								<p class="text-sm text-gray-500">
									Temperature: {selectedPromptConfig.temperature}
								</p>
								<div class="dropdown dropdown-end dropdown-right dropdown-hover">
									<div tabindex="0" role="button" class="link-hover link text-gray-400">
										Show System Prompt
									</div>
									<div
										class="card dropdown-content compact z-[1] ml-1 w-64 rounded-box bg-base-200 shadow"
									>
										<div class="card-body">
											<p>{selectedPromptConfig.system_prompt}</p>
										</div>
									</div>
								</div>
							{/if}
						</div>
					</div>
				</label>
			</div>
			<div class="divider mb-0"></div>
			<!--  Favorite Modal Prompt Toggle Save -->
			<div class="form-control flex-row items-center justify-end gap-2">
				<span class="label-text overflow-hidden" style="max-height: 3lh;"
					>Save to favorites: e.g. Use a cost-effective(GPT-4o-mini) API as the normal option, and
					save advanced/pricer(4o or o1) models as favorite.</span
				>
				<div class="join">
					{#each llmState.defaultFavModels as value}
						<button
							type="button"
							class="btn join-item btn-sm"
							class:btn-active={activeFavId === value}
							onclick={() => (activeFavId = value)}
						>
							{value === 0 ? 'Default' : value}
						</button>
					{/each}
					<input type="hidden" name="selected_fav_model" value={activeFavId} />
				</div>
			</div>
			<!-- Save Button -->
			<div class="modal-action items-center gap-4">
				<p>
					Go to <a href="/llmSettings/prompt" class="link-hover text-accent"
						>API & Prompt Settings</a
					>
					page to edit these options or add new!
				</p>
				<button class="btn_emerald">Save</button>
			</div>
		</form>
	</div>
</dialog>

<style lang="postcss">
	.select {
		@apply bg-base-300;
	}
</style>
