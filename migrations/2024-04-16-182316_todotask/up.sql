-- Your SQL goes here
CREATE TABLE todotasks (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INT NOT NULL,
    todolist_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    parent_task_id INT,
    due_date DATE,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL,
    modified_at TIMESTAMP NOT NULL
)
