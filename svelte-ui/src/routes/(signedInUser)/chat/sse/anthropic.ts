import Anthropic from '@anthropic-ai/sdk';

export const clientRequest = (previous_chat: ChatMessage[], query: string, favModel: FavModel) => {
	const previousMessages: Anthropic.Messages.MessageParam[] = previous_chat.flatMap((message) => [
		{ role: 'user', content: message.user_query },
		{ role: 'assistant', content: message.assistant_response }
	]);
	const client = new Anthropic({ apiKey: favModel.api.secret_key });

	previousMessages.push({ role: 'user', content: query });
	const messages = previousMessages;

	const stream = client.messages.stream({
		model: favModel.model,
		max_tokens: favModel.prompt.max_tokens,
		temperature: favModel.prompt.temperature,
		system: favModel.prompt.system_prompt,
		messages: messages
	});

	return stream;
};
