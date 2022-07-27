-- Add up migration script here
CREATE TABLE IF NOT EXISTS tags(
	id SERIAL PRIMARY KEY,
	user_id INT REFERENCES users(id),
	name VARCHAR(12) NOT NULL
);
CREATE UNIQUE INDEX unique_user_tag ON tags(user_id, name);
