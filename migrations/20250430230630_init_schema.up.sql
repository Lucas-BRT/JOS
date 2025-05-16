-- Extensions
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Enums
CREATE TYPE user_role AS ENUM ('user', 'admin');

CREATE TYPE attendance_status AS ENUM ('unknown', 'confirmed', 'absent');

CREATE TYPE session_status AS ENUM ('planned', 'confirmed', 'cancelled', 'finished');

CREATE TYPE session_frequency AS ENUM ('weekly', 'biweekly', 'monthly');

-- Genres
CREATE TABLE IF NOT EXISTS game_genres (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    category TEXT NOT NULL
);

-- Systems
CREATE TABLE IF NOT EXISTS systems (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Users
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    username TEXT UNIQUE NOT NULL,
    display_name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    user_role user_role DEFAULT 'user' NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Tables (RPG groups)
CREATE TABLE IF NOT EXISTS tables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    gm_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    system_id INT NOT NULL REFERENCES systems (id),
    contact_info TEXT NOT NULL,
    max_players INT,
    language TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Table Genres (many-to-many)
CREATE TABLE IF NOT EXISTS table_genres (
    table_id UUID NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
    genre_id INT NOT NULL REFERENCES game_genres (id) ON DELETE CASCADE,
    PRIMARY KEY (table_id, genre_id)
);

-- Table Participants
CREATE TABLE IF NOT EXISTS table_participants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    table_id UUID NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, table_id)
);

-- Campaigns
CREATE TABLE IF NOT EXISTS campaigns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    table_id UUID NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Adventures
CREATE TABLE IF NOT EXISTS adventures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    campaign_id UUID NOT NULL REFERENCES campaigns (id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Sessions
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    table_id UUID NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
    campaign_id UUID REFERENCES campaigns (id) ON DELETE SET NULL,
    adventure_id UUID REFERENCES adventures (id) ON DELETE SET NULL,
    scheduled_at TIMESTAMPTZ NOT NULL,
    duration_minutes INT,
    title TEXT,
    summary TEXT,
    is_one_shot BOOLEAN DEFAULT FALSE,
    status session_status DEFAULT 'planned',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Session Attendance
CREATE TABLE IF NOT EXISTS session_attendance (
    session_id UUID NOT NULL REFERENCES sessions (id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    status attendance_status DEFAULT 'unknown',
    confirmed_by UUID REFERENCES users (id),
    confirmed_at TIMESTAMPTZ,
    PRIMARY KEY (session_id, user_id)
);

-- Table Schedule
CREATE TABLE IF NOT EXISTS table_schedule (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    table_id UUID NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
    weekday SMALLINT NOT NULL CHECK (weekday BETWEEN 0 AND 6),
    time TIME NOT NULL,
    frequency session_frequency NOT NULL
);
