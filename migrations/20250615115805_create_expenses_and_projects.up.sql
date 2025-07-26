-- Add up migration script here

CREATE TABLE projects (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  name character varying NOT NULL,
  created_at timestamp default current_timestamp NOT NULL,
--   total_expenses double precision NOT NULL,
  currency VARCHAR(255) NOT NULL
);

CREATE TYPE expense_type AS ENUM ('expense', 'transfer', 'gain');

CREATE TABLE expenses (
  id SERIAL PRIMARY KEY,
  author_id integer REFERENCES users(id) NOT NULL,
  project_id uuid REFERENCES projects(id) NOT NULL,
  created_at timestamp default current_timestamp NOT NULL,
  amount double precision NOT NULL,
  description character varying NULL,
  name character varying NOT NULL,
  expense_type expense_type NOT NULL
);