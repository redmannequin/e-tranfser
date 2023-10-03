CREATE TABLE IF NOT EXISTS payments (
  payment_id uuid NOT NULL PRIMARY KEY,
  payer_full_name VARCHAR(255) NOT NULL,
  payer_email VARCHAR(255) NOT NULL,
  payee_full_name VARCHAR(255) NOT NULL,
  payee_email VARCHAR(255) NOT NULL,
  amount INTEGER NOT NULL,
  security_question VARCHAR(255),
  security_answer VARCHAR(255),
  state SMALLINT NOT NULL
);
