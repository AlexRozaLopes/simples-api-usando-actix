-- Your SQL goes here
CREATE TABLE livros (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  author TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
);
