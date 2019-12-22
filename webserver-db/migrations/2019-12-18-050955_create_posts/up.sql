-- Your SQL goes here
CREATE TABLE pages (
    id SERIAL PRIMARY KEY,
    parent_id INT NOT NULL,
    name VARCHAR NOT NULL,
    body TEXT NOT NULL,
    is_folder BOOLEAN NOT NULL DEFAULT 'f',
    published BOOLEAN NOT NULL DEFAULT 'f'
)