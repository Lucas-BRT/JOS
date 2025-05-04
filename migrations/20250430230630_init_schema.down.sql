-- Add down migration script here
-- Drop many-to-many first
DROP TABLE IF EXISTS table_participants;

DROP TABLE IF EXISTS table_genres;

-- Drop dependent tables
DROP TABLE IF EXISTS tables;

DROP TABLE IF EXISTS users;

-- Drop lookup/reference tables
DROP TABLE IF EXISTS systems;

DROP TABLE IF EXISTS game_genres;

-- Drop enums
DROP TYPE IF EXISTS user_role;
