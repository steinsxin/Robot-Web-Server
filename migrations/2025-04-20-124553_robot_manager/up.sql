-- Your SQL goes here
CREATE TABLE robot_manager (
    id SERIAL PRIMARY KEY,
    robot_id VARCHAR NOT NULL,
    electricity INT NOT NULL,
    activate BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at timestamp not null default now()
);