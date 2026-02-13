ALTER TABLE expenses ADD COLUMN date DATE;
UPDATE expenses SET date = created_at::date;
ALTER TABLE expenses ALTER COLUMN date SET NOT NULL;
