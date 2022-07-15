-- Your SQL goes here
CREATE TABLE users
(
    id         SERIAL PRIMARY KEY, 
    url        VARCHAR NOT NULL, 
    token      VARCHAR NOT NULL, 
)