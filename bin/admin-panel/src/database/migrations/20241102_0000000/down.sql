-- Drop the d_accounts table if it exists
DROP TABLE IF EXISTS d_accounts;

-- Drop the d_users table if it exists
DROP TABLE IF EXISTS d_users;

DROP INDEX IF EXISTS idx_unique_secret;
DROP INDEX IF EXISTS idx_unique_owner_name;
DROP TABLE IF EXISTS d_projects;

DROP TABLE IF EXISTS t_project_invites;
DROP TABLE IF EXISTS d_project_members;