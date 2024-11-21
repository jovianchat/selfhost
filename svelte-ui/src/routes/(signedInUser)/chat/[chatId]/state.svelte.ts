class GlobalChatState {
	qr: ChatMessage[] = $state([]);
	starred = $state(false);
	title = $state('');
	isResponseGenerating = $state(false);

	emptyQR() {
		this.qr = [];
	}
	addQuery(user_query: string) {
		this.qr.push({ user_query, assistant_response: '' });
	}
}

export const chatState = new GlobalChatState();
