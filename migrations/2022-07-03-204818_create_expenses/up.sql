-- Your SQL goes here

CREATE TABLE projects (
  id integer PRIMARY KEY,
  name character varying NOT NULL,
  created_at date NOT NULL,
  total_expenses double precision NOT NULL,
  currency VARCHAR(255)
);

CREATE TABLE expenses (
  id integer PRIMARY KEY,
  paid_for_id integer REFERENCES users(id),
  paid_by_id integer REFERENCES users(id),
  author_id integer REFERENCES users(id),
  project_id integer REFERENCES projects(id),
  date date NOT NULL,
  amount double precision NOT NULL,
  description character varying NULL,
  name character varying NOT NULL,
  expense_type VARCHAR(255)
);