import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const chatId = params.chatId;

	const res = await fetch(`/axum-api/chat/${chatId}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});
	if (res.ok) {
		const chat: Chat = await res.json();
		return { chat };
	} else {
		throw new Error(await res.text());
	}
};
