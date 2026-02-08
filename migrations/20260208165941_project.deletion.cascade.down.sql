-- Add down migration script here
ALTER TABLE expenses DROP CONSTRAINT expenses_project_id_fkey;
ALTER TABLE payments DROP CONSTRAINT payments_expense_id_fkey;
ALTER TABLE user_projects DROP CONSTRAINT user_projects_project_id_fkey;

-- Should Recreate original FK ?