-- Add up migration script here
CREATE TABLE IF NOT EXISTS transactions(
	id SERIAL PRIMARY KEY,
	creation_time TIMESTAMP WITHOUT TIME ZONE DEFAULT (now() AT TIME ZONE('utc')),
	account_id INT REFERENCES accounts(id),
	amount DECIMAL NOT NULL,
	cost BOOL NOT NULL default 'f',
	commentary VARCHAR(100),
	obligatory BOOL
);
CREATE TABLE IF NOT EXISTS transaction_tags(
	transaction_id INT REFERENCES transactions(id),
	tag_id INT REFERENCES tags(id),
	PRIMARY KEY (transaction_id, tag_id)
);
