-- Your SQL goes here
CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	did text not null unique
)

