CREATE TABLE IF NOT EXISTS t_project_invites (
    id VARCHAR(255) PRIMARY KEY,
    project_id VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    role VARCHAR(8) DEFAULT 'MEMBER',
    expire_at BIGINT DEFAULT 0,

    CONSTRAINT fk_invite_project FOREIGN KEY (project_id) REFERENCES d_projects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS d_project_members (
    id SERIAL PRIMARY KEY,
    project_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    role VARCHAR(8) NOT NULL,

    CONSTRAINT fk_invite_project FOREIGN KEY (project_id) REFERENCES d_projects(id) ON DELETE CASCADE
);