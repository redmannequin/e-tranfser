CREATE TABLE IF NOT EXISTS payments (
  payment_id uuid NOT NULL PRIMARY KEY,
  payer_full_name VARCHAR(255) NOT NULL,
  payer_email VARCHAR(255) NOT NULL,
  payee_full_name VARCHAR(255) NOT NULL,
  payee_email VARCHAR(255) NOT NULL,
  amount INTEGER NOT NULL,
  security_question VARCHAR(255),
  security_answer VARCHAR(255),
  payment_state SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  user_id uuid NOT NULL PRIMARY KEY,
  first_name: VARCHAR(255) NOT NULL,
  last_name: VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  primary_account_id uuid,
);

CREATE TABLE IF NOT EXISTS user_accounts (
  account_id uuid NOT NULL,
  user_id uuid NOT NULL,
  provider_id INTEGER NOT NULL,
  iban VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS account_providers (
  provider_id INTEGER NOT NULL,
  provider_name VARCHAR(255) NOT NULL,
);

CREATE TABLE IF NOT EXISTS user_payees (
  payee_id uuid NOT NULL PRIMARY KEY,
  user_id uuid NOT NULL,
  full_name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  phone_number VARCHAR(255) NOT NULL
);

CREATE TABLE If NOT EXISTS user_payments (
  payment_id uuid NOT NULL PRIMARY KEY,
  user_id uuid NOT NULL,
  payee_id uuid NOT NULL,
  amount INTEGER NOT NULL
  security_question VARCHAR(255),
  security_answer VARCHAR(255),
  payment_state SMALLINT NOT NULL
);