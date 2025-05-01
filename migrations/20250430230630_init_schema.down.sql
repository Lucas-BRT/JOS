-- Add down migration script here
-- DROP Tables
DROP TABLE IF EXISTS table_participants;

DROP TABLE IF EXISTS table_genres;

DROP TABLE IF EXISTS tables;

DROP TABLE IF EXISTS users;

DROP TABLE IF EXISTS systems;

DROP TABLE IF EXISTS genres;

-- DROP Enums
DROP TYPE IF EXISTS user_role;

DROP TYPE IF EXISTS user_gender;

DROP TYPE IF EXISTS game_theme;
