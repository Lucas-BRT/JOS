-- ENUMs
CREATE TYPE access_level AS ENUM ('user', 'admin', 'moderator');

CREATE TYPE attendance_status AS ENUM ('unknown', 'confirmed', 'absent');

CREATE TYPE session_status AS ENUM ('planned', 'confirmed', 'cancelled', 'finished');

CREATE TYPE request_status AS ENUM ('pending', 'approved', 'rejected', 'cancelled');

CREATE TYPE player_experience AS ENUM ('beginner', 'intermediate', 'advanced', 'expert');

CREATE TYPE table_status AS ENUM (
    -- mean that the master is not yet begin the table
    'planned',
    -- mean that the table is running and active
    'active',
    -- mean that the table is finished and completed
    'finished',
    -- for some reason the GM temporarily archived the table
    'archived',
    -- means the table is permanently deleted
    'deleted'
);

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    access_level access_level NOT NULL,
    bio TEXT,
    avatar_url TEXT,
    nickname TEXT,
    years_of_experience INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

-- Table for game systems
CREATE TABLE IF NOT EXISTS game_systems (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

-- Tables table
CREATE TABLE IF NOT EXISTS tables (
    id UUID PRIMARY KEY,
    gm_id UUID NOT NULL REFERENCES users (id),
    title TEXT NOT NULL,
    game_system_id UUID NOT NULL REFERENCES game_systems (id),
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    description TEXT NOT NULL,
    player_slots INTEGER NOT NULL,
    bg_image_link TEXT,
    recommended_player_experience player_experience,
    status table_status NOT NULL DEFAULT 'planned',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

-- Requests table
CREATE TABLE IF NOT EXISTS requests (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users (id),
    table_id UUID NOT NULL REFERENCES tables (id),
    status request_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY,
    table_id UUID NOT NULL REFERENCES tables (id),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    status session_status NOT NULL DEFAULT 'planned',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);
