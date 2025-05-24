-- Extensions
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Enums
CREATE TYPE user_role AS ENUM ('user', 'admin');
CREATE TYPE attendance_status AS ENUM ('unknown', 'confirmed', 'absent');
CREATE TYPE session_status AS ENUM ('planned', 'confirmed', 'cancelled', 'finished');
CREATE TYPE request_status AS ENUM ('pending', 'approved', 'rejected', 'cancelled');

-- Genres (Gêneros)
CREATE TABLE IF NOT EXISTS game_genres (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL, -- Ex: "Medieval", "Sombria", "Espacial"
    category TEXT NOT NULL, -- Ex: "Fantasia", "Horror", "Ficção Científica"
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, -- Adicionado para consistência
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, -- Adicionado para consistência
    UNIQUE (name, category) -- Um gênero específico dentro de uma categoria deve ser único
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
    system_id INT NOT NULL REFERENCES systems (id) ON DELETE RESTRICT, -- Impede deletar sistema em uso
    contact_info TEXT NOT NULL,
    max_players INT CHECK (max_players IS NULL OR max_players > 0), -- Garante que max_players é positivo
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Table Genres (many-to-many)
CREATE TABLE IF NOT EXISTS table_genres (
    table_id UUID NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
    genre_id INT NOT NULL REFERENCES game_genres (id) ON DELETE CASCADE,
    PRIMARY KEY (table_id, genre_id)
);

-- Campaigns
CREATE TABLE IF NOT EXISTS campaigns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    table_id UUID NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP -- Adicionado para consistência
);

-- Adventures
CREATE TABLE IF NOT EXISTS adventures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    title TEXT NOT NULL,
    description TEXT,
    campaign_id UUID REFERENCES campaigns (id) ON DELETE CASCADE,
    table_id UUID REFERENCES tables (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, -- Adicionado para consistência
    CONSTRAINT chk_adventure_parent CHECK (
        (campaign_id IS NOT NULL AND table_id IS NULL) OR
        (campaign_id IS NULL AND table_id IS NOT NULL)
    )
);

-- Sessions
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    adventure_id UUID REFERENCES adventures (id) ON DELETE CASCADE,
    campaign_id UUID REFERENCES campaigns (id) ON DELETE CASCADE,
    table_id UUID REFERENCES tables (id) ON DELETE CASCADE,
    scheduled_at TIMESTAMPTZ NOT NULL,
    duration_minutes INT CHECK (duration_minutes IS NULL OR duration_minutes > 0),
    title TEXT,
    summary TEXT,
    is_one_shot BOOLEAN DEFAULT FALSE,
    status session_status DEFAULT 'planned' NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, -- Adicionado para consistência
    CONSTRAINT chk_session_parent CHECK (
        (adventure_id IS NOT NULL AND campaign_id IS NULL AND table_id IS NULL) OR
        (adventure_id IS NULL AND campaign_id IS NOT NULL AND table_id IS NULL) OR
        (adventure_id IS NULL AND campaign_id IS NULL AND table_id IS NOT NULL)
    )
);

-- Table Join Requests
CREATE TABLE IF NOT EXISTS table_join_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    table_id UUID NOT NULL REFERENCES tables(id) ON DELETE CASCADE,
    message TEXT,
    status request_status DEFAULT 'pending' NOT NULL,
    requested_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    resolved_at TIMESTAMPTZ,
    resolver_id UUID REFERENCES users(id) ON DELETE SET NULL,
    -- Impede múltiplas solicitações com o MESMO status do mesmo usuário para a mesma mesa.
    -- Para impedir múltiplas solicitações PENDENTES, um índice parcial seria mais preciso:
    -- CREATE UNIQUE INDEX ON table_join_requests (user_id, table_id) WHERE (status = 'pending');
    -- Por ora, mantendo a constraint mais simples:
    CONSTRAINT uq_user_table_status_request UNIQUE (user_id, table_id, status)
);

-- Table Participants
CREATE TABLE IF NOT EXISTS table_participants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (), -- Adicionado ID próprio para a participação
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    table_id UUID NOT NULL REFERENCES tables (id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    UNIQUE (user_id, table_id)
);

-- Session Attendance
CREATE TABLE IF NOT EXISTS session_attendance (
    session_id UUID NOT NULL REFERENCES sessions (id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    status attendance_status DEFAULT 'unknown' NOT NULL,
    confirmed_by UUID REFERENCES users (id) ON DELETE SET NULL, -- Quem confirmou (pode ser o próprio ou GM)
    confirmed_at TIMESTAMPTZ,
    PRIMARY KEY (session_id, user_id)
);

-- Triggers para updated_at (opcional, mas boa prática)
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Aplicar triggers às tabelas relevantes
CREATE TRIGGER set_timestamp_game_genres BEFORE UPDATE ON game_genres FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_systems BEFORE UPDATE ON systems FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_users BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_tables BEFORE UPDATE ON tables FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_campaigns BEFORE UPDATE ON campaigns FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_adventures BEFORE UPDATE ON adventures FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
CREATE TRIGGER set_timestamp_sessions BEFORE UPDATE ON sessions FOR EACH ROW EXECUTE FUNCTION trigger_set_timestamp();
