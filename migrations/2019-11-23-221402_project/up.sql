-- Your SQL goes here
CREATE TABLE project (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    complete INTEGER NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE issue (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    project_id INTEGER NOT NULL,
    complete INTEGER NOT NULL,
    content TEXT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (project_id)
        REFERENCES project (id)
);