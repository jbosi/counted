CREATE TABLE accounts (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  password_hash VARCHAR NOT NULL,
  display_name VARCHAR NOT NULL,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL
);
