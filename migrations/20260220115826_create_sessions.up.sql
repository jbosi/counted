CREATE TABLE sessions (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  account_id UUID REFERENCES accounts(id) ON DELETE CASCADE NOT NULL,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  expires_at TIMESTAMP NOT NULL
);
