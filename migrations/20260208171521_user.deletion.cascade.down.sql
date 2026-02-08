-- Add down migration script here
ALTER TABLE user_projects DROP CONSTRAINT IF EXISTS user_projects_user_id_fkey;
