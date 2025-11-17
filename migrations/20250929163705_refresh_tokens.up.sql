-- Add up migration script here
CREATE TABLE refresh_tokens
(
    id         UUID PRIMARY KEY NOT NULL,
    user_id    UUID             NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token      TEXT             NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ      NOT NULL,
    created_at TIMESTAMPTZ      NOT NULL DEFAULT now()
);

CREATE INDEX refresh_tokens_token ON refresh_tokens (token);
