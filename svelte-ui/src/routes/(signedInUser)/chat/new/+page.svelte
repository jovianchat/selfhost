<script lang="ts">
	import { submitQuery, textArea } from '../textArea.svelte';

	let defaultPrompts = [
		'What is the weather like today?',
		'Tell me a joke.',
		'Explain quantum physics in simple terms.',
		'How to cook pasta?',
		"What's the latest news?",
		"Translate 'Hello' to Spanish.",
		'How do I learn programming?',
		'What are the top movies right now?',
		'How many planets are in the solar system?'
	];
	let chatId = 'new';
	function handleSubmit(event: any, prompt: string) {
		event.preventDefault();
		textArea.value = prompt;
		submitQuery(chatId);
	}
</script>

<div class="my-auto flex flex-col">
	<h2 class="mx-auto mb-4 text-xl font-bold">Choose a Default Prompt</h2>
	<div class="grid grid-cols-3 gap-4">
		{#each defaultPrompts as prompt}
			<button
				class="rounded-lg bg-emerald-600 bg-opacity-70 p-3 text-white shadow transition-all hover:bg-emerald-800"
				onclick={(e) => handleSubmit(e, prompt)}
			>
				{prompt}
			</button>
		{/each}
	</div>
</div>
