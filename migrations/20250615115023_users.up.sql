-- Add up migration script here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name character varying NOT NULL,
  balance double precision,
  created_at timestamp default current_timestamp
);