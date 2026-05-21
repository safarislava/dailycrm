CREATE TABLE IF NOT EXISTS stages (
    project_id UUID NOT NULL,
    id UUID DEFAULT gen_random_uuid(),
    position INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    deadline TIMESTAMP,
    cost INTEGER,
    PRIMARY KEY (project_id, id),
    FOREIGN KEY (project_id) REFERENCES projects(id)
);