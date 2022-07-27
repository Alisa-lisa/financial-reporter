-- Add up migration script here
CREATE TABLE IF NOT EXISTS users(
	id SERIAL PRIMARY KEY,
	did TEXT NOT NULL UNIQUE
)
