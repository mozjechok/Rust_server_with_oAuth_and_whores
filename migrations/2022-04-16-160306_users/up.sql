-- Your SQL goes here
CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    login      VARCHAR NOT NULL UNIQUE,
    password   VARCHAR NOT NULL,
    fio        VARCHAR NOT NULL,
    unik       VARCHAR NOT NULL,
    grade      VARCHAR NOT NULL,
    birthdate  VARCHAR NOT NULL
)