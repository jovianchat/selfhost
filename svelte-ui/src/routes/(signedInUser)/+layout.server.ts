import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies, fetch }) => {
	// Log out if token expired
	const refresh_token = cookies.get('refresh_token');
	const expiration = cookies.get('refresh_token_expiration');
	const currentTime = Math.ceil(Date.now() / 1000);
	if (!refresh_token || !expiration || Number(expiration) < currentTime) {
		return redirect(302, '/auth');
	}

	let chats: ChatHistory;
	const llmSettings: LLMSettings = {
		apiConfigs: [],
		promptConfigs: [],
		favModels: []
	};
	// if (sidebar.chatHistory.length === 0) {
	const res = await fetch(`/axum-api/chat/history`);
	if (res.ok) {
		chats = await res.json();
	} else {
		throw new Error(await res.text());
	}
	// }
	/* Load LLM Api & Models and Prompt Configs for selector and to be sent with each user_query as to which model to use and what prompt settings */
	const res_apiConfigs = await fetch('/axum-api/llm-settings/api-config', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});
	if (!res_apiConfigs.ok) {
		throw new Error(await res_apiConfigs.text());
	}
	const res_promptConfigs = await fetch('/axum-api/llm-settings/prompt-engineering', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});
	if (!res_promptConfigs.ok) {
		throw new Error(await res_promptConfigs.text());
	}
	const res_favModels = await fetch('/axum-api/llm-settings/fav-models', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});
	if (!res_favModels.ok) {
		throw new Error(await res_favModels.text());
	}

	llmSettings.apiConfigs = await res_apiConfigs.json();
	llmSettings.promptConfigs = await res_promptConfigs.json();
	llmSettings.favModels = await res_favModels.json();

	return { chats, llmSettings };
};
