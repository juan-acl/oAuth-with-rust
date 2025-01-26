-- Your SQL goes here

CREATE TABLE session (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INT NOT NULL,
    token TEXT NOT NULL,
    token_valid TINYINT NOT NULL DEFAULT 1
);