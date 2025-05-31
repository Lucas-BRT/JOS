-- Drop tables na ordem inversa para respeitar FKs
DROP TABLE IF EXISTS sessions;

DROP TABLE IF EXISTS requests;

DROP TABLE IF EXISTS tables;

DROP TABLE IF EXISTS users;

-- Drop types ENUM
DROP TYPE IF EXISTS request_status;

DROP TYPE IF EXISTS session_status;

DROP TYPE IF EXISTS attendance_status;

DROP TYPE IF EXISTS access_level;

-- Drop extension pgcrypto (normalmente não se remove, mas para completo)
DROP EXTENSION IF EXISTS "pgcrypto";
