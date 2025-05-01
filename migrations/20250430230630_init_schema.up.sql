-- Enums
CREATE TYPE user_role AS ENUM ('user', 'admin');
CREATE TYPE user_gender AS ENUM ('male', 'female', 'other');
CREATE TYPE game_theme AS ENUM ('fantasy', 'sci-fi', 'horror', 'modern', 'historical');

-- Genres
CREATE TABLE IF NOT EXISTS genres (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    category VARCHAR(100) NOT NULL
);

-- Systems
CREATE TABLE IF NOT EXISTS systems (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Users
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(30) UNIQUE NOT NULL CHECK (username ~ '^[a-zA-Z0-9_]{3,30}$'),
    display_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role DEFAULT 'user',
    gender user_gender DEFAULT 'other',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Tables (RPG groups)
CREATE TABLE IF NOT EXISTS tables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    gm_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    description TEXT,
    system_id INT NOT NULL REFERENCES systems (id),
    theme game_theme NOT NULL,
    contact_info VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Table Genres (many-to-many)
CREATE TABLE IF NOT EXISTS table_genres (
    table_id UUID NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
    genre_id INT NOT NULL REFERENCES genres (id) ON DELETE CASCADE,
    PRIMARY KEY (table_id, genre_id)
);

-- Participants
CREATE TABLE IF NOT EXISTS table_participants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    table_id UUID NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, table_id)
);
