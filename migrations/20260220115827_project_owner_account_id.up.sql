ALTER TABLE projects ADD COLUMN owner_account_id UUID REFERENCES accounts(id) ON DELETE SET NULL;
