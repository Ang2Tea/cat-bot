-- Add down migration script here
ALTER TABLE chats DROP COLUMN enable_push; 
ALTER TABLE chats DROP COLUMN title; 