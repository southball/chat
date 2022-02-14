CREATE TABLE Accounts (
  user_guid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username TEXT UNIQUE,
  google_email TEXT UNIQUE
);
