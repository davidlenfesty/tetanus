-- Your SQL goes here
CREATE TABLE pages (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    body TEXT NOT NULL,
    isfolder BOOLEAN NOT NULL DEFAULT 'f',
    published BOOLEAN NOT NULL DEFAULT 'f'
)