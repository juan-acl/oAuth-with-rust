-- Your SQL goes here
CREATE TABLE user (
    id INT PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    lastname TEXT NOT NULL,
    email TEXT NOT NULL,
    address TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    password TEXT NOT NULL
);
