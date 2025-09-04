-- SQL Down Migration Script

-- Drop triggers first
DROP TRIGGER IF EXISTS update_table_requests_updated_at ON table_requests;
DROP TRIGGER IF EXISTS update_session_checkins_updated_at ON session_checkins;
DROP TRIGGER IF EXISTS update_session_intents_updated_at ON session_intents;
DROP TRIGGER IF EXISTS update_sessions_updated_at ON sessions;
DROP TRIGGER IF EXISTS update_table_members_updated_at ON table_members;
DROP TRIGGER IF EXISTS update_tables_updated_at ON tables;
DROP TRIGGER IF EXISTS update_users_updated_at ON users;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop indexes
DROP INDEX IF EXISTS idx_table_requests_status;
DROP INDEX IF EXISTS idx_table_requests_user_id;
DROP INDEX IF EXISTS idx_table_requests_table_id;
DROP INDEX IF EXISTS idx_session_checkins_intent_id;
DROP INDEX IF EXISTS idx_session_intents_status;
DROP INDEX IF EXISTS idx_session_intents_user_id;
DROP INDEX IF EXISTS idx_session_intents_session_id;
DROP INDEX IF EXISTS idx_sessions_accepting_intents;
DROP INDEX IF EXISTS idx_sessions_scheduled_for;
DROP INDEX IF EXISTS idx_sessions_table_id;
DROP INDEX IF EXISTS idx_table_members_status;
DROP INDEX IF EXISTS idx_table_members_user_id;
DROP INDEX IF EXISTS idx_table_members_table_id;
DROP INDEX IF EXISTS idx_tables_status;
DROP INDEX IF EXISTS idx_tables_gm_id;

-- Drop tables in reverse order (due to foreign key constraints)
DROP TABLE IF EXISTS table_requests CASCADE;
DROP TABLE IF EXISTS session_checkins CASCADE;
DROP TABLE IF EXISTS session_intents CASCADE;
DROP TABLE IF EXISTS sessions CASCADE;
DROP TABLE IF EXISTS table_members CASCADE;
DROP TABLE IF EXISTS tables CASCADE;
DROP TABLE IF EXISTS game_systems CASCADE;
DROP TABLE IF EXISTS users CASCADE;

-- Drop enum types in reverse order
DROP TYPE IF EXISTS request_status;
DROP TYPE IF EXISTS intent_status;
DROP TYPE IF EXISTS session_status;
DROP TYPE IF EXISTS table_status;
DROP TYPE IF EXISTS table_visibility;