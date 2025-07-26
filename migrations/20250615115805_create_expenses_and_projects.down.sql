-- Add down migration script here

DROP TABLE IF EXISTS expenses;
DROP TABLE IF EXISTS projects;
DROP TYPE IF EXISTS expense_type;