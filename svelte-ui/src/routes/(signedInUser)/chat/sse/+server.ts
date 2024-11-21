import type { RequestHandler } from '@sveltejs/kit';
import type { SseRequestBody } from '../[chatId]/chatResponse';
import { clientRequest as openaiClientRequest } from './openai';
import { clientRequest as openaiCompatibleClientRequest } from './openai_compatible';
import { clientRequest as anthropicClientRequest } from './anthropic';
import { LlmSdk } from '../../llmSettings/state.svelte';

export const POST: RequestHandler = async ({ request, fetch }) => {
	try {
		const { chatId, selected_fav_model_id, query, isTitleGenerating, isNewChat }: SseRequestBody =
			await request.json();

		let previous_chat: ChatMessage[] = [];
		if (!(isTitleGenerating || isNewChat)) {
			const res_chat = await fetch(`/axum-api/chat/${chatId}`, {
				method: 'GET',
				headers: {
					'Content-Type': 'application/json'
				}
			});
			if (!res_chat.ok) {
				throw new Error(await res_chat.text());
			}
			previous_chat = ((await res_chat.json()) as Chat).messages;
		} else {
			previous_chat = [];
		}

		const res_favModel = await fetch(`/axum-api/llm-settings/fav-model/${selected_fav_model_id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		});
		if (!res_favModel.ok) {
			throw new Error(await res_favModel.text());
		}
		const favModel: FavModel = await res_favModel.json();
		if (isTitleGenerating) {
			favModel.prompt.max_tokens = 10;
			favModel.prompt.temperature = 0.1;
			favModel.prompt.system_prompt =
				'This query is for LLM Chat. You generate titles for these chats with strictly less than 6 words & one sentence only.';
		}

		let stream;
		if (favModel.api.endpoint_sdk === LlmSdk.OpenAI) {
			stream = openaiClientRequest(previous_chat, query, favModel);
		} else if (favModel.api.endpoint_sdk === LlmSdk.Anthropic) {
			stream = anthropicClientRequest(previous_chat, query, favModel);
		} else if (favModel.api.endpoint_sdk === LlmSdk.OpenAICompatible) {
			stream = openaiCompatibleClientRequest(previous_chat, query, favModel);
		} else {
			throw new Error('Unsupported endpoint SDK');
		}

		return new Response(stream.toReadableStream(), {
			headers: {
				'Content-Type': 'text/event-stream',
				'Cache-Control': 'no-cache',
				Connection: 'keep-alive'
			}
		});
	} catch (error) {
		console.error('Error:', error);
		return new Response('Error processing request', { status: 500 });
	}
};

// Set up Server-Sent Events
// const stream = new ReadableStream({
//     async start(controller) {
//         for await (const chunk of response) {
//             const content = chunk.choices[0].delta.content;
//             if (content) {
//                 controller.enqueue(content);
//             }
//         }
//         controller.close();
//     },
//     cancel() {
//         // cancel your resources here
//     }
// });
