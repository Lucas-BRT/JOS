-- Initialize JOS Database
-- This script runs when the PostgreSQL container starts for the first time

-- Create extensions if they don't exist
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create additional databases for different environments
CREATE DATABASE jos_test WITH OWNER = postgres;
CREATE DATABASE jos_staging WITH OWNER = postgres;

-- Grant necessary permissions
GRANT ALL PRIVILEGES ON DATABASE jos_dev TO postgres;
GRANT ALL PRIVILEGES ON DATABASE jos_test TO postgres;
GRANT ALL PRIVILEGES ON DATABASE jos_staging TO postgres;

-- Create a dedicated user for the application (optional)
-- CREATE USER jos_app WITH PASSWORD 'jos_app_password';
-- GRANT CONNECT ON DATABASE jos_dev TO jos_app;
-- GRANT USAGE ON SCHEMA public TO jos_app;
-- GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO jos_app;
-- GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO jos_app;

-- Set default privileges for future objects
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO postgres;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO postgres;

-- Log the initialization
SELECT 'JOS Database initialized successfully' as status;
