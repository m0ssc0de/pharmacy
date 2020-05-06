-- Your SQL goes here
CREATE TABLE medicines (
  id VARCHAR NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  descr TEXT NOT NULL,
  price INT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)