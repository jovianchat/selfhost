import type { ChatCompletionMessageParam } from 'openai/resources/index.mjs';
import { OpenAI } from 'openai';

export const clientRequest = (previous_chat: ChatMessage[], query: string, favModel: FavModel) => {
	const previousMessages: ChatCompletionMessageParam[] = previous_chat.flatMap((message) => [
		{ role: 'user', content: message.user_query },
		{ role: 'assistant', content: message.assistant_response }
	]);
	const client = new OpenAI({ apiKey: favModel.api.secret_key, baseURL: favModel.api.base_url });

	const messages: ChatCompletionMessageParam[] = [
		{ role: 'system', content: favModel.prompt.system_prompt },
		...previousMessages,
		{ role: 'user', content: query }
	];

	const stream = client.beta.chat.completions.stream({
		model: favModel.model,
		max_completion_tokens: favModel.prompt.max_tokens,
		temperature: favModel.prompt.temperature,
		messages: messages,
		stream: true
	});

	return stream;
};
