ALTER TABLE accounts
  DROP COLUMN failed_login_count,
  DROP COLUMN locked_until;
