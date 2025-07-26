-- Add up migration script here

CREATE TABLE user_projects (
  project_id uuid REFERENCES projects(id),
  user_id INTEGER REFERENCES users(id),
  PRIMARY KEY(project_id, user_id)
);