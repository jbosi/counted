-- Your SQL goes here

CREATE TABLE payments (
  id integer PRIMARY KEY,
  expense_id integer REFERENCES expenses(id) NOT NULL,
  user_id integer REFERENCES users(id) NOT NULL,
  is_debt boolean NOT NULL,
  amount double precision NOT NULL,
  created_at timestamp default current_timestamp
);