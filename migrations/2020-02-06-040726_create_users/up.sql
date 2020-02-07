-- Your SQL goes here
CREATE TABLE rusty_users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    rusty_password VARCHAR NOT NULL,
    rusty_role VARCHAR NOT NULL
)