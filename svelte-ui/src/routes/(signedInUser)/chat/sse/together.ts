import Together from 'together-ai';

export const clientRequest = (previous_chat: ChatMessage[], query: string, favModel: FavModel) => {
	const previousMessages: Together.Chat.Completions.CompletionCreateParams.Message[] =
		previous_chat.flatMap((message) => [
			{ role: 'user', content: message.user_query },
			{ role: 'assistant', content: message.assistant_response }
		]);
	const client = new Together({ apiKey: favModel.api.secret_key });

	const messages: Together.Chat.Completions.CompletionCreateParams.Message[] = [
		{ role: 'system', content: favModel.prompt.system_prompt },
		...previousMessages,
		{ role: 'user', content: query }
	];

	const stream = client.chat.completions.stream({
		model: favModel.model,
		max_tokens: favModel.prompt.max_tokens,
		temperature: favModel.prompt.temperature,
		messages: messages,
		stream: true
	});

	return stream;
};
