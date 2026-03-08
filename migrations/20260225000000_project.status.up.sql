CREATE TYPE project_status AS ENUM ('ongoing', 'closed', 'archived');
ALTER TABLE projects ADD COLUMN status project_status NOT NULL DEFAULT 'ongoing';
