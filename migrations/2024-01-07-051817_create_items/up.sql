-- Your SQL goes here
CREATE TABLE items (
  id VARCHAR NOT NULL PRIMARY KEY,
  room_id VARCHAR NOT NULL REFERENCES rooms(id) on DELETE CASCADE,
  name VARCHAR NOT NULL,
  description VARCHAR,
  category VARCHAR NOT NULL,
  purchase_date Timestamp NOT NULL,
  expiry_date Timestamp,
  value DOUBLE PRECISION NOT NULL
)
