import { sidebarState } from '$/components/layout/sidebar.svelte';
import { goto } from '$app/navigation';
import { ChatCompletionStream } from 'openai/lib/ChatCompletionStream.mjs';
import { chatState } from './[chatId]/state.svelte';
import { generateResponse, type SseRequestBody } from './[chatId]/chatResponse';
import { MessageStream } from '@anthropic-ai/sdk/lib/MessageStream.mjs';
import { LlmSdk, llmState } from '../llmSettings/state.svelte';

class TextAreaState {
	value = $state('');
}
export const textArea = new TextAreaState();

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export async function submitQuery(chatId: any) {
	const sseRequestBody: SseRequestBody = {
		chatId: chatId,
		query: textArea.value,
		selected_fav_model_id: llmState.activeFav!.id
	};
	// New chat
	if (chatId === 'new') {
		try {
			sseRequestBody.isNewChat = true;
			sseRequestBody.isTitleGenerating = true;
			const res = await fetch('/chat/sse', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(sseRequestBody)
			});
			let title;
			if (llmState.activeFav!.api.endpoint_sdk === LlmSdk.Anthropic) {
				// @ts-expect-error ReadableStream on different environments can be strange
				const runner = MessageStream.fromReadableStream(res.body);
				title = await runner.finalText();
			} else {
				// @ts-expect-error ReadableStream on different environments can be strange
				const runner = ChatCompletionStream.fromReadableStream(res.body);
				title = await runner.finalContent();
			}
			// Save title to DB and get title from DB
			const access_token = await (await fetch('/hooks_fetchHandler')).text();
			const resId = await fetch(`/axum-api/chat/new`, {
				method: 'POST',
				headers: {
					'Content-Type': 'text/plain',
					Authorization: `Bearer ${access_token}`
				},
				body: title
			});
			if (resId.ok) {
				const { id, title }: HistoryChatDetails = await resId.json();
				sseRequestBody.chatId = id;
				sidebarState.addUnstarredChatToHistory({ id, title });
				goto(`/chat/${id}`);
			} else {
				throw new Error(await resId.text());
			}
		} finally {
			sseRequestBody.isTitleGenerating = false;
		}
	}
	chatState.addQuery(textArea.value);
	textArea.value = '';
	// Generate response for both existing and new chat
	generateResponse(sseRequestBody);
}
