import type { Actions } from '@sveltejs/kit';

export const actions: Actions = {
	default: async ({ request, fetch }) => {
		const formData = await request.formData();
		const password = formData.get('new-password') as string;
		const confirmPassword = formData.get('confirm-password') as string;
		if (password !== confirmPassword) {
			return { error: 'Passwords do not match' };
		}
		const res = await fetch('/axum-api/auth/change-password', {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(password)
		});
		if (res.ok) {
			return { success: true };
		} else {
			throw new Error(await res.text());
		}
	}
};
