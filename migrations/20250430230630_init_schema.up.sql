CREATE TYPE "e_roles" AS ENUM (
	'admin',
	'moderator',
	'user'
);

CREATE TYPE "e_table_visibility" AS ENUM (
	'private',
	'public'
);

CREATE TYPE "e_intent_status" AS ENUM (
	'yes',
	'no',
	'maybe'
);

CREATE TABLE "t_users" (
	"id" UUID NOT NULL,
	"name" TEXT NOT NULL UNIQUE,
	"email" TEXT NOT NULL UNIQUE,
	"password_hash" TEXT NOT NULL,
	"role" E_ROLES NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ,
	PRIMARY KEY("id")
);




CREATE TABLE "t_rpg_tables" (
	"id" UUID NOT NULL,
	"gm_id" UUID NOT NULL,
	"title" TEXT NOT NULL,
	"visibility" E_TABLE_VISIBILITY NOT NULL,
	"description" TEXT NOT NULL,
	"game_system_id" UUID NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ,
	PRIMARY KEY("id")
);




CREATE TABLE "t_game_system" (
	"id" UUID NOT NULL,
	"name" TEXT NOT NULL UNIQUE,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	PRIMARY KEY("id")
);




CREATE TABLE "t_sessions" (
	"id" UUID NOT NULL,
	"name" TEXT NOT NULL,
	"table_id" UUID NOT NULL,
	"accepting_intents" BOOLEAN NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ,
	PRIMARY KEY("id")
);


CREATE INDEX "t_sessions_index_0"
ON "t_sessions" ("table_id");

CREATE TABLE "t_session_intents" (
	"id" UUID NOT NULL,
	"user_id" UUID NOT NULL,
	"session_id" UUID NOT NULL,
	"intent_status" E_INTENT_STATUS NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ,
	PRIMARY KEY("user_id", "session_id")
);


CREATE INDEX "t_session_intents_index_0"
ON "t_session_intents" ("session_id");

CREATE TABLE "t_session_checkins" (
	"id" UUID NOT NULL,
	"session_intent_id" UUID NOT NULL,
	"attendance" BOOLEAN NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ,
	PRIMARY KEY("id")
);



ALTER TABLE "t_rpg_tables"
ADD FOREIGN KEY("gm_id") REFERENCES "t_users"("id")
ON UPDATE NO ACTION ON DELETE CASCADE;
ALTER TABLE "t_rpg_tables"
ADD FOREIGN KEY("game_system_id") REFERENCES "t_game_system"("id")
ON UPDATE NO ACTION ON DELETE CASCADE;
ALTER TABLE "t_sessions"
ADD FOREIGN KEY("table_id") REFERENCES "t_rpg_tables"("id")
ON UPDATE NO ACTION ON DELETE CASCADE;
ALTER TABLE "t_session_intents"
ADD FOREIGN KEY("user_id") REFERENCES "t_users"("id")
ON UPDATE NO ACTION ON DELETE CASCADE;
ALTER TABLE "t_session_checkins"
ADD FOREIGN KEY("session_intent_id") REFERENCES "t_session_intents"("id")
ON UPDATE NO ACTION ON DELETE CASCADE;
ALTER TABLE "t_session_intents"
ADD FOREIGN KEY("session_id") REFERENCES "t_sessions"("id")
ON UPDATE NO ACTION ON DELETE CASCADE;
