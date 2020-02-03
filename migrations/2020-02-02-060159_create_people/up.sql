-- Your SQL goes here
CREATE TABLE people (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  age INT NOT NULL,
  sex VARCHAR NOT NULL
)
