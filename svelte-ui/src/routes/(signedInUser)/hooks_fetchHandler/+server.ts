import type { RequestEvent } from '@sveltejs/kit';

export async function GET({ fetch, cookies }: RequestEvent) {
	// Health check to refresh the access token before sending sse request
	const healthCheck = await fetch('/axum-api/server-health');
	if (!healthCheck.ok) {
		throw new Error(healthCheck.statusText);
	}
	const access_token = cookies.get('access_token');
	return new Response(access_token);
}
