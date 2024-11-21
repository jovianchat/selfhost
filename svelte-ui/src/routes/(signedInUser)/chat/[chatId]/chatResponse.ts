import { chatState } from './state.svelte';
import { ChatCompletionStream } from 'openai/lib/ChatCompletionStream';
import { LlmSdk, llmState } from '../../llmSettings/state.svelte';
import { MessageStream } from '@anthropic-ai/sdk/resources/messages.mjs';

export interface SseRequestBody {
	chatId: string;
	query: string;
	selected_fav_model_id: number;
	isTitleGenerating?: boolean;
	isNewChat?: boolean;
}
export async function generateResponse(sseRequestBody: SseRequestBody) {
	chatState.isResponseGenerating = true;
	const res = await fetch('/chat/sse', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(sseRequestBody)
	});
	let runner;
	if (llmState.activeFav!.api.endpoint_sdk === LlmSdk.Anthropic) {
		// @ts-expect-error ReadableStream on different environments can be strange
		runner = MessageStream.fromReadableStream(res.body);
		runner.on('text', (text) => {
			chatState.qr[chatState.qr.length - 1].assistant_response += text;
		});
	} else {
		// @ts-expect-error ReadableStream on different environments can be strange
		runner = ChatCompletionStream.fromReadableStream(res.body);
		runner.on('content', (delta, snapshot) => {
			// process.stdout.write(delta);
			// or, in a browser, you might display like this:
			// document.body.innerText += delta; // or:
			// document.body.innerText = snapshot;
			// chatState.addResponse(delta);
			chatState.qr[chatState.qr.length - 1].assistant_response = snapshot;
		});
	}

	const collectResponse = await runner.finalMessage();
	// const collectResponse = (await runner.finalChatCompletion()).choices[0].message.content;
	if (collectResponse) {
		await esClose_SaveDb(sseRequestBody.chatId);
	} else {
		throw new Error('No response');
	}
	// const access_token = await (await fetch('/hooks_fetchHandler')).text();
	// if (browser) {
	// 	eventSource = new EventSource(
	// 		`/axum-api/chat-sse?chat_id=${chatId}&access_token=${access_token}&selected_fav_model_id=${textArea.activeFav}&query=${encodeURIComponent(query)}`
	// 	);
	// 	eventSource.onmessage = async (event) => {
	// 		if (event.data === 'End of Stream') {
	// 			await esClose_SaveDb(chatId);
	// 			return;
	// 		}
	// 		chatState.addResponse(event.data);
	// 	};
	// 	eventSource.onerror = async () => {
	// 		await esClose_SaveDb(chatId);
	// 	};
	// }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export async function esClose_SaveDb(chatId: any) {
	const access_token = await (await fetch('/hooks_fetchHandler')).text();
	if (chatState.isResponseGenerating) {
		chatState.isResponseGenerating = false;
		const currentMessage: ChatMessage = chatState.qr[chatState.qr.length - 1];
		const res = await fetch(`/axum-api/chat/${chatId}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${access_token}`
			},
			body: JSON.stringify(currentMessage)
		});
		if (!res.ok) {
			throw new Error(await res.text());
		}
	}
}
