<script lang="ts">

	import { llmState } from "../routes/(signedInUser)/llmSettings/state.svelte";

</script>

<div class="sticky bottom-0 mt-auto bg-base-100">
    <div class="mt-4">
        <div class="flex flex-col">
            <div class="textarea textarea-md mb-4 flex flex-col items-center justify-center">
                <span class="text-base text-gray-500">Favorite Models:</span>
                <div class="join">
                    {#each llmState.defaultFavModels as favModelId}
                        <!-- If for btn-disabled if these  -->
                        {#if llmState.settings.favModels.some((model) => model.id === favModelId)}
                            <button
                                type="button"
                                class="btn join-item btn-sm"
                                class:btn-active={llmState.activeFav?.id === favModelId}
                                onclick={() => llmState.setActiveFavById(favModelId)}
                            >
                                {favModelId === 0 ? 'Default' : favModelId}
                            </button>
                        {:else}
                            <button type="button" class="btn btn-disabled join-item btn-sm">
                                {favModelId === 0 ? 'Default' : favModelId}
                            </button>
                        {/if}
                    {/each}
                </div>
            </div>
        </div>
    </div>
</div>