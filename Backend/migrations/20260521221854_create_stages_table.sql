CREATE TABLE IF NOT EXISTS stages (
    project_id UUID NOT NULL,
    position INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    deadline TIMESTAMP,
    cost INTEGER,
    PRIMARY KEY (project_id, position),
    FOREIGN KEY (project_id) REFERENCES projects(id)
);