-- Your SQL goes here

CREATE TABLE project_users (
  id SERIAL PRIMARY KEY,
  project_id integer REFERENCES projects(id) NOT NULL,
  user_id integer REFERENCES users(id) NOT NULL,
  created_at timestamp default current_timestamp NOT NULL
);