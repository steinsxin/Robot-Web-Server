-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    context TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);