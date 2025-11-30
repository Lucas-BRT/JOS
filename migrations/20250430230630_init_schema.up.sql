CREATE TYPE session_status AS ENUM ('scheduled', 'in_progress', 'completed', 'cancelled');
CREATE TYPE intent_status AS ENUM ('confirmed', 'unsure', 'declined');
CREATE TYPE request_status AS ENUM ('pending', 'approved', 'rejected');
CREATE TYPE table_status AS ENUM ('active', 'inactive');

CREATE TABLE users (
    "id" UUID NOT NULL DEFAULT uuidv7(),
    "username" TEXT NOT NULL UNIQUE,
    "email" TEXT NOT NULL UNIQUE,
    "password" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id")
);

CREATE TABLE game_systems (
    "id" UUID NOT NULL DEFAULT uuidv7(),
    "name" TEXT NOT NULL UNIQUE,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id")
);

CREATE TABLE tables (
    "id" UUID NOT NULL DEFAULT uuidv7(),
    "gm_id" UUID NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "slots" INTEGER NOT NULL CHECK ("slots" >= 0),
    "status" table_status NOT NULL DEFAULT 'active',
    "game_system_id" UUID NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("gm_id") REFERENCES users("id") ON DELETE CASCADE,
    FOREIGN KEY("game_system_id") REFERENCES game_systems("id") ON DELETE CASCADE
);

CREATE TABLE table_members (
    "id" UUID NOT NULL DEFAULT uuidv7(),
    "table_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("table_id") REFERENCES tables("id") ON DELETE CASCADE,
    FOREIGN KEY("user_id") REFERENCES users("id") ON DELETE CASCADE,
    UNIQUE("table_id", "user_id")
);

CREATE TABLE sessions (
    "id" UUID NOT NULL DEFAULT uuidv7(),
    "title" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "table_id" UUID NOT NULL,
    "scheduled_for" TIMESTAMPTZ,
    "status" session_status NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("table_id") REFERENCES tables("id") ON DELETE CASCADE
);

CREATE TABLE session_intents (
    "id" UUID NOT NULL DEFAULT uuidv7(),
    "user_id" UUID NOT NULL,
    "session_id" UUID NOT NULL,
    "intent_status" intent_status NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("user_id") REFERENCES users("id") ON DELETE CASCADE,
    FOREIGN KEY("session_id") REFERENCES sessions("id") ON DELETE CASCADE,
    UNIQUE("user_id", "session_id")
);

CREATE TABLE session_checkins (
    "id" UUID NOT NULL DEFAULT uuidv7(),
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
    "id" UUID NOT NULL DEFAULT uuidv7(),
    "user_id" UUID NOT NULL,
    "table_id" UUID NOT NULL,
    "message" TEXT,
    "status" request_status NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY("id"),
    FOREIGN KEY("user_id") REFERENCES users("id") ON DELETE CASCADE,
    FOREIGN KEY("table_id") REFERENCES tables("id") ON DELETE CASCADE
);

CREATE INDEX idx_tables_gm_id ON tables ("gm_id");
CREATE INDEX idx_table_members_table_id ON table_members ("table_id");
CREATE INDEX idx_table_members_user_id ON table_members ("user_id");
CREATE INDEX idx_sessions_table_id ON sessions ("table_id");
CREATE INDEX idx_sessions_scheduled_for ON sessions ("scheduled_for");
CREATE INDEX idx_sessions_status ON sessions ("status");
CREATE INDEX idx_session_intents_session_id ON session_intents ("session_id");
CREATE INDEX idx_session_intents_user_id ON session_intents ("user_id");
CREATE INDEX idx_session_intents_status ON session_intents ("intent_status");
CREATE INDEX idx_session_checkins_intent_id ON session_checkins ("session_intent_id");
CREATE INDEX idx_table_requests_table_id ON table_requests ("table_id");
CREATE INDEX idx_table_requests_user_id ON table_requests ("user_id");
CREATE INDEX idx_table_requests_status ON table_requests ("status");
