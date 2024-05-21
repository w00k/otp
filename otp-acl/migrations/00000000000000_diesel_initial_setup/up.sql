CREATE TABLE IF NOT EXISTS otp_keys (
  id SERIAL PRIMARY KEY,
  otp_public_key VARCHAR NOT NULL,
  otp_private_key VARCHAR NOT NULL,
  otp_user VARCHAR NOT NULL,
  retry INTEGER NOT NULL,
  expiration_date TIMESTAMP NOT NULL DEFAULT NOW(),
  otp_key_enable BOOLEAN NOT NULL DEFAULT TRUE
);

