-- Drop the existing FK
ALTER TABLE expenses DROP CONSTRAINT IF EXISTS expenses_project_id_fkey;
-- Re‑create with cascade
ALTER TABLE expenses
  ADD CONSTRAINT expenses_project_id_fkey
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE;

-- Drop the existing FK
ALTER TABLE payments DROP CONSTRAINT IF EXISTS payments_expense_id_fkey;
-- Re‑create with cascade
ALTER TABLE payments
  ADD CONSTRAINT payments_expense_id_fkey
  FOREIGN KEY (expense_id) REFERENCES expenses(id) ON DELETE CASCADE;

-- Drop the existing FK
ALTER TABLE user_projects DROP CONSTRAINT IF EXISTS user_projects_project_id_fkey;
-- Re‑create with cascade
ALTER TABLE user_projects
  ADD CONSTRAINT user_projects_project_id_fkey
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE;