-- Create 'query_response' Table
CREATE TABLE ai.chat_messages (
    chat_id UUID NOT NULL REFERENCES ai.chats(id) ON DELETE CASCADE,
    user_query TEXT NOT NULL,
    assistant_response TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (chat_id, created_at)
);

CREATE INDEX idx_chat_id ON ai.chat_messages(chat_id);
CREATE INDEX idx_created_at ON ai.chat_messages(created_at);

CREATE OR REPLACE FUNCTION update_chat_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE ai.chats
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.chat_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_chat_timestamp_trigger
AFTER INSERT ON ai.chat_messages
FOR EACH ROW
EXECUTE FUNCTION update_chat_timestamp();
