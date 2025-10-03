-- Add up migration script here
CREATE TABLE IF NOT EXISTS games (
    id UUID PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    creator TEXT NOT NULL,
    plays INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
)
