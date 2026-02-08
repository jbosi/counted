-- Drop the existing FK
ALTER TABLE user_projects DROP CONSTRAINT IF EXISTS user_projects_user_id_fkey;
-- Re‑create with cascade
ALTER TABLE user_projects
  ADD CONSTRAINT user_projects_user_id_fkey
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;