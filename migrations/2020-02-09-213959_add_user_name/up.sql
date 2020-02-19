-- Your SQL goes here
ALTER TABLE rusty_users
ADD COLUMN rusty_user_name VARCHAR NOT NULL UNIQUE;