-- Add up migration script here

ALTER TABLE projects
ADD COLUMN description VARCHAR(250);