CREATE TABLE IF NOT EXISTS "d_projects" (
    "id" VARCHAR(255) PRIMARY KEY,
    "owner" VARCHAR(255) NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "secret" VARCHAR(255) NOT NULL,
    "options" JSONB,
    "codecs" JSONB,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_unique_secret ON d_projects("secret");
CREATE UNIQUE INDEX IF NOT EXISTS idx_unique_owner_name ON d_projects("owner", "name");