CREATE TABLE IF NOT EXISTS payments (
  payment_id uuid NOT NULL PRIMARY KEY,
  full_name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  amount INTEGER NOT NULL,
  security_question VARCHAR(100),
  security_answer VARCHAR(255)
);
