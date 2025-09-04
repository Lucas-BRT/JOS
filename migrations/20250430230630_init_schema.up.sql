CREATE TYPE table_visibility AS ENUM ('private', 'public');
CREATE TYPE table_status AS ENUM ('draft', 'open', 'paused', 'completed', 'cancelled');
CREATE TYPE session_status AS ENUM ('scheduled', 'in_progress', 'completed', 'cancelled');
CREATE TYPE intent_status AS ENUM ('confirmed', 'tentative', 'declined');
CREATE TYPE request_status AS ENUM ('pending', 'approved', 'rejected');

CREATE TABLE users (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "username" TEXT NOT NULL UNIQUE,
    "display_name" TEXT NOT NULL,
    "email" TEXT NOT NULL UNIQUE,
    "password" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id")
);

CREATE TABLE game_systems (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "name" TEXT NOT NULL UNIQUE,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id")
);

CREATE TABLE tables (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "gm_id" UUID NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "visibility" table_visibility NOT NULL DEFAULT 'public',
    "player_slots" INTEGER NOT NULL CHECK ("player_slots" >= 0),
    "game_system_id" UUID NOT NULL,
    "status" table_status NOT NULL DEFAULT 'draft',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("gm_id") REFERENCES users("id") ON DELETE CASCADE,
    FOREIGN KEY("game_system_id") REFERENCES game_systems("id") ON DELETE CASCADE
);

CREATE TABLE table_members (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "table_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "role" TEXT NOT NULL DEFAULT 'player',
    "joined_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "status" TEXT NOT NULL DEFAULT 'active',
    "character_name" TEXT,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("table_id") REFERENCES tables("id") ON DELETE CASCADE,
    FOREIGN KEY("user_id") REFERENCES users("id") ON DELETE CASCADE,
    UNIQUE("table_id", "user_id")
);

CREATE TABLE sessions (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "table_id" UUID NOT NULL,
    "scheduled_for" TIMESTAMPTZ,
    "status" session_status NOT NULL DEFAULT 'scheduled',
    "accepting_intents" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("table_id") REFERENCES tables("id") ON DELETE CASCADE
);

CREATE TABLE session_intents (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "user_id" UUID NOT NULL,
    "session_id" UUID NOT NULL,
    "intent_status" intent_status NOT NULL DEFAULT 'tentative',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("user_id") REFERENCES users("id") ON DELETE CASCADE,
    FOREIGN KEY("session_id") REFERENCES sessions("id") ON DELETE CASCADE,
    UNIQUE("user_id", "session_id")
);

CREATE TABLE session_checkins (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "session_intent_id" UUID NOT NULL,
    "attendance" BOOLEAN NOT NULL,
    "notes" TEXT,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("session_intent_id") REFERENCES session_intents("id") ON DELETE CASCADE,
    UNIQUE("session_intent_id")
);

CREATE TABLE table_requests (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "user_id" UUID NOT NULL,
    "table_id" UUID NOT NULL,
    "message" TEXT,
    "status" request_status NOT NULL DEFAULT 'pending',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("user_id") REFERENCES users("id") ON DELETE CASCADE,
    FOREIGN KEY("table_id") REFERENCES tables("id") ON DELETE CASCADE
);

CREATE INDEX idx_tables_gm_id ON tables ("gm_id");
CREATE INDEX idx_tables_status ON tables ("status");
CREATE INDEX idx_table_members_table_id ON table_members ("table_id");
CREATE INDEX idx_table_members_user_id ON table_members ("user_id");
CREATE INDEX idx_table_members_status ON table_members ("status");
CREATE INDEX idx_sessions_table_id ON sessions ("table_id");
CREATE INDEX idx_sessions_scheduled_for ON sessions ("scheduled_for");
CREATE INDEX idx_sessions_accepting_intents ON sessions ("accepting_intents");
CREATE INDEX idx_session_intents_session_id ON session_intents ("session_id");
CREATE INDEX idx_session_intents_user_id ON session_intents ("user_id");
CREATE INDEX idx_session_intents_status ON session_intents ("intent_status");
CREATE INDEX idx_session_checkins_intent_id ON session_checkins ("session_intent_id");
CREATE INDEX idx_table_requests_table_id ON table_requests ("table_id");
CREATE INDEX idx_table_requests_user_id ON table_requests ("user_id");
CREATE INDEX idx_table_requests_status ON table_requests ("status");

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tables_updated_at
    BEFORE UPDATE ON tables
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_table_members_updated_at
    BEFORE UPDATE ON table_members
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sessions_updated_at
    BEFORE UPDATE ON sessions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_session_intents_updated_at
    BEFORE UPDATE ON session_intents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_session_checkins_updated_at
    BEFORE UPDATE ON session_checkins
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_table_requests_updated_at
    BEFORE UPDATE ON table_requests
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();