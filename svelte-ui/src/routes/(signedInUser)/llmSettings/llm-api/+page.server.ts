import type { LlmSdk } from '../state.svelte';
import type { Actions } from './$types';

export const actions: Actions = {
	saveApiConfig: async ({ request, fetch }) => {
		const formData = await request.formData();
		const apiConfig: ApiConfig = {
			id: '',
			name: formData.get('name') as string,
			endpoint_sdk: formData.get('endpoint-sdk') as LlmSdk,
			base_url: formData.get('base-url') as string,
			secret_key: formData.get('api-key') as string,
			models: formData.getAll('models') as string[]
		};

		const id = formData.get('id');
		let res_method;
		if (!id) {
			// Create new llm api config
			const apiConfigsLength = formData.get('apiConfigsLength');
			apiConfig.id = Number(apiConfigsLength);
			res_method = 'POST';
		} else {
			// Update existing llm api config
			apiConfig.id = Number(id);
			res_method = 'PATCH';
		}
		const res = await fetch('/axum-api/llm-settings/api-config', {
			method: res_method,
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(apiConfig)
		});
		if (res.ok) {
			return { success: true };
		} else {
			throw new Error(await res.text());
		}
	},
	deleteApiConfig: async ({ request, fetch }) => {
		const formData = await request.formData();
		const id = formData.get('id') as string;
		const res = await fetch('/axum-api/llm-settings/api-config', {
			method: 'DELETE',
			headers: {
				'Content-Type': 'text/plain'
			},
			body: id
		});
		if (res.ok) {
			return { success: true };
		} else {
			throw new Error(await res.text());
		}
	}
};
