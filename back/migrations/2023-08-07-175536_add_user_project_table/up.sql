-- Your SQL goes here

CREATE TABLE user_projects (
  id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
  user_id integer REFERENCES users(id) NOT NULL,
  project_id uuid REFERENCES projects(id) NOT NULL
);