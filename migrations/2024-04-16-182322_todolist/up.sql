-- Your SQL goes here
CREATE TABLE todolists (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id SERIAL NOT NULL,
    shared_with TEXT,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    modified_at TIMESTAMP NOT NULL
)