-- Your SQL goes here
CREATE TABLE todolists (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INT NOT NULL,
    shared_with TEXT,
    parent_list_id INT,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    modified_at TIMESTAMP NOT NULL
)