-- Add up migration script here
CREATE TABLE IF NOT EXISTS accounts(
	id SERIAL PRIMARY KEY,
	user_id INT REFERENCES users(id),
	name VARCHAR(25) NOT NULL
);
CREATE UNIQUE INDEX unique_account ON accounts(user_id, name);
