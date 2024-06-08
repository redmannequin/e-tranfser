CREATE TABLE IF NOT EXISTS payments (
  payment_id UUID NOT NULL PRIMARY KEY,
  data_version INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  payment_data JSONB not NULL DEFAULT '{}'
);

CREATE TABLE IF NOT EXISTS payout_id_to_payment_id (
  payout_id UUID NOT NULL PRIMARY KEY,
  payment_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  CONSTRAINT fk_payment
    FOREIGN KEY(payment_id) 
    REFERENCES payments(payment_id)
);

CREATE TABLE IF NOT EXISTS refund_id_to_payment_id (
  refund_id UUID NOT NULL PRIMARY KEY,
  payment_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  CONSTRAINT fk_payment
    FOREIGN KEY(payment_id) 
    REFERENCES payments(payment_id)
);

CREATE TABLE IF NOT EXISTS users (
  user_id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(255) NOT NULL,
  data_version INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  user_data JSONB not NULL DEFAULT '{}'
);

CREATE TABLE If NOT EXISTS user_payments (
  payment_id UUID NOT NULL PRIMARY KEY,
  user_id UUID NOT NULL,
  data_version INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  payment_data JSONB not NULL DEFAULT '{}'
);