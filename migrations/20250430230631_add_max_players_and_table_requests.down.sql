-- Remover índices da tabela t_table_requests
DROP INDEX IF EXISTS t_table_requests_index_2;
DROP INDEX IF EXISTS t_table_requests_index_1;
DROP INDEX IF EXISTS t_table_requests_index_0;

-- Remover tabela t_table_requests
DROP TABLE IF EXISTS t_table_requests;

-- Remover campo max_players da tabela t_rpg_tables
ALTER TABLE t_rpg_tables DROP COLUMN IF EXISTS "max_players";
