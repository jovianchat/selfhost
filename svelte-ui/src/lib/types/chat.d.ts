import type { UUID } from 'crypto';

declare global {
	interface HistoryChatDetails {
		id: UUID;
		title: string;
	}
	interface UnstarredGroupedHistory {
		time_period: string;
		period_chats: HistoryChatDetails[];
	}
	interface ChatHistory {
		unstarred_history: UnstarredGroupedHistory[];
		starred_history: HistoryChatDetails[];
	}

	// Each chat
	interface ChatDetails {
		id: UUID;
		title: string;
		starred: boolean;
	}
	interface ChatMessage {
		user_query: string;
		assistant_response: string;
	}
	interface Chat {
		details: ChatDetails;
		messages: ChatMessage[];
	}
}
