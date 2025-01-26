-- Your SQL goes here
CREATE TABLE user (
    id INTEGER NOT NULL PRIMARY KEY  AUTOINCREMENT  ,
    name TEXT NOT NULL,
    lastname TEXT NOT NULL,
    email TEXT NOT NULL,
    address TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    password TEXT NOT NULL,
    active TINYINT NOT NULL DEFAULT 0
);
