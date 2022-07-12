-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name character varying NOT NULL,
  balance double precision
);