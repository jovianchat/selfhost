class GlobalSidebarState {
	isOpen = $state(true);
	chatHistory: ChatHistory = $state({
		unstarred_history: [],
		starred_history: []
	});
	toggle(localStorage: Storage) {
		this.isOpen = !this.isOpen;
		localStorage.setItem('isSidebarOpen', this.isOpen.toString());
	}
	initHistory(chats: ChatHistory) {
		this.chatHistory = chats;
	}
	addUnstarredChatToHistory(chat: HistoryChatDetails) {
		if (this.chatHistory.unstarred_history.length === 0) {
			this.chatHistory.unstarred_history.push({
				time_period: 'Recent',
				period_chats: [chat]
			});
		} else {
			this.chatHistory.unstarred_history[0].period_chats.unshift(chat);
		}
	}
}
export const sidebarState = new GlobalSidebarState();

declare global {
	interface Array<T> {
		myShiftFilter(predicate: (value: T) => boolean): void;
	}
}
Array.prototype.myShiftFilter = function <T>(this: T[], predicate: (value: T) => boolean): void {
	let i, j;

	for (i = 0, j = 0; i < this.length; ++i) {
		if (predicate(this[i])) {
			this[j] = this[i];
			++j;
		}
	}
	while (j < this.length) {
		this.pop();
	}
};
