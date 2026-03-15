CREATE TABLE account_projects (
  account_id UUID    NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  project_id UUID    NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  user_id    INTEGER,
  PRIMARY KEY (account_id, project_id)
);
