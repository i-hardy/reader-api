-- Your SQL goes here
CREATE TABLE canonical_feeds (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  url VARCHAR NOT NULL
)