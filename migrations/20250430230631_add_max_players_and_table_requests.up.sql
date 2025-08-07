-- Adicionar campo max_players na tabela t_rpg_tables
ALTER TABLE t_rpg_tables ADD COLUMN "max_players" INTEGER NOT NULL DEFAULT 4;

-- Criar tabela para requisições de jogadores que querem entrar em uma mesa
CREATE TABLE t_table_requests (
	"id" UUID NOT NULL,
	"user_id" UUID NOT NULL,
	"table_id" UUID NOT NULL,
	"message" TEXT,
	"status" TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected')),
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ,
	PRIMARY KEY("id"),
	UNIQUE("user_id", "table_id"),
	FOREIGN KEY("user_id") REFERENCES t_users("id") ON UPDATE NO ACTION ON DELETE CASCADE,
	FOREIGN KEY("table_id") REFERENCES t_rpg_tables("id") ON UPDATE NO ACTION ON DELETE CASCADE
);

-- Criar índices para melhor performance
CREATE INDEX t_table_requests_index_0 ON t_table_requests ("table_id");
CREATE INDEX t_table_requests_index_1 ON t_table_requests ("user_id");
CREATE INDEX t_table_requests_index_2 ON t_table_requests ("status");
