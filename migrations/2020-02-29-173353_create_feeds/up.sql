-- Your SQL goes here
CREATE TABLE feeds (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  website VARCHAR NOT NULL,
  last_updated TIMESTAMPTZ NOT NULL,
  canonical_feed INTEGER REFERENCES canonical_feeds(id) NOT NULL
)