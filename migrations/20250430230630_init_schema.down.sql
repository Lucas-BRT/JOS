-- Remover Triggers (se aplicável, liste todas as tabelas que usaram)
DROP TRIGGER IF EXISTS set_timestamp_sessions ON sessions;

DROP TRIGGER IF EXISTS set_timestamp_adventures ON adventures;

DROP TRIGGER IF EXISTS set_timestamp_campaigns ON campaigns;

DROP TRIGGER IF EXISTS set_timestamp_tables ON tables;

DROP TRIGGER IF EXISTS set_timestamp_users ON users;

DROP TRIGGER IF EXISTS set_timestamp_systems ON systems;

DROP TRIGGER IF EXISTS set_timestamp_game_genres ON game_genres;

-- Remover Função de Trigger
DROP FUNCTION IF EXISTS trigger_set_timestamp ();

-- Remover Tabelas na ordem inversa de criação e dependências
DROP TABLE IF EXISTS session_attendance;

DROP TABLE IF EXISTS table_participants;

DROP TABLE IF EXISTS table_join_requests;

DROP TABLE IF EXISTS sessions;

DROP TABLE IF EXISTS adventures;

DROP TABLE IF EXISTS campaigns;

DROP TABLE IF EXISTS table_genres;

DROP TABLE IF EXISTS tables;

DROP TABLE IF EXISTS users;

DROP TABLE IF EXISTS systems;

DROP TABLE IF EXISTS game_genres;

-- Remover Enums (Tipos)
DROP TYPE IF EXISTS request_status;

DROP TYPE IF EXISTS session_status;

DROP TYPE IF EXISTS attendance_status;

DROP TYPE IF EXISTS user_role;

-- A extensão pgcrypto geralmente não é removida, pois pode ser usada por outros bancos/schemas.
-- Se precisar remover: DROP EXTENSION IF EXISTS "pgcrypto";
