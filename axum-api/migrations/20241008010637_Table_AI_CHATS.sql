-- Create 'chat' Schema 
CREATE SCHEMA IF NOT EXISTS ai;

CREATE TABLE ai.chats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    starred BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create Indexes for Performance Optimization
CREATE INDEX idx_starred ON ai.chats(starred);
CREATE INDEX idx_updated_at ON ai.chats(updated_at);