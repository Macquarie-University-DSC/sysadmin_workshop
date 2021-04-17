SET TIME ZONE 'UTC';

DROP TABLE IF EXISTS todo;

DROP SCHEMA IF EXISTS testing CASCADE;

CREATE TABLE todo (
    id serial UNIQUE NOT NULL PRIMARY KEY,
    description varchar (255) NOT NULL,
    is_complete boolean NOT NULL,
    issue_date timestamp NOT NULL
);

CREATE SCHEMA testing;

CREATE TABLE testing.todo (
    id serial UNIQUE NOT NULL PRIMARY KEY,
    description varchar (255) NOT NULL,
    is_complete boolean NOT NULL,
    issue_date timestamp NOT NULL
);

INSERT INTO testing.todo (description, is_complete, issue_date) VALUES ('test todo 1', FALSE, '2021-04-17 20:00:00');
INSERT INTO testing.todo (description, is_complete, issue_date) VALUES ('test todo 2', FALSE, '2021-04-17 21:00:00');
INSERT INTO testing.todo (description, is_complete, issue_date) VALUES ('test todo 3', TRUE, '2021-04-17 10:00:00');
INSERT INTO testing.todo (description, is_complete, issue_date) VALUES ('test todo 4', TRUE, '2021-04-17 11:00:00');

