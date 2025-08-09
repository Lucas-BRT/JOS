# Sistema de Mapeamento de Constraints

Este documento descreve o sistema de mapeamento automático de constraints do PostgreSQL para variantes específicas da enum `RepositoryError`.

## Visão Geral

O sistema foi projetado para:
- Mapear automaticamente violações de constraints UNIQUE para erros específicos
- Mapear violações de FOREIGN KEY para erros estruturados
- Extrair valores dos campos violados das mensagens de erro do PostgreSQL
- Fornecer fallback para constraints não reconhecidas

## Estrutura

### Módulo `constraint_mapper`

Localizado em `src/infrastructure/repositories/constraint_mapper.rs`, contém:

- `map_constraint_violation()`: Mapeia constraints específicas para RepositoryError
- `map_database_error()`: Função principal que processa sqlx::Error
- `extract_field_from_error()`: Extrai valores dos campos violados

### Variantes da RepositoryError

```rust
pub enum RepositoryError {
    // UNIQUE constraints
    UsernameAlreadyTaken(String),
    EmailAlreadyTaken(String),
    GameSystemNameAlreadyTaken(String),
    UserSessionIntentAlreadyExists,
    
    // FOREIGN KEY constraints
    ForeignKeyViolation { table: String, field: String },
    
    // Fallback
    UnknownConstraint(String),
    
    // Outros
    DatabaseError(sqlx::Error),
    UserNotFound,
}
```

## Constraints Mapeadas

### UNIQUE Constraints

| Constraint | Tabela | Campo | RepositoryError |
|------------|--------|-------|-----------------|
| `t_users_name_key` | `t_users` | `name` | `UsernameAlreadyTaken` |
| `t_users_email_key` | `t_users` | `email` | `EmailAlreadyTaken` |
| `t_game_system_name_key` | `t_game_system` | `name` | `GameSystemNameAlreadyTaken` |
| `t_session_intents_user_id_session_id_key` | `t_session_intents` | `(user_id, session_id)` | `UserSessionIntentAlreadyExists` |

### FOREIGN KEY Constraints

| Constraint | Tabela | Campo | Referência | RepositoryError |
|------------|--------|-------|------------|-----------------|
| `t_rpg_tables_gm_id_fkey` | `t_rpg_tables` | `gm_id` | `t_users(id)` | `ForeignKeyViolation` |
| `t_rpg_tables_game_system_id_fkey` | `t_rpg_tables` | `game_system_id` | `t_game_system(id)` | `ForeignKeyViolation` |
| `t_sessions_table_id_fkey` | `t_sessions` | `table_id` | `t_rpg_tables(id)` | `ForeignKeyViolation` |
| `t_session_intents_user_id_fkey` | `t_session_intents` | `user_id` | `t_users(id)` | `ForeignKeyViolation` |
| `t_session_intents_session_id_fkey` | `t_session_intents` | `session_id` | `t_sessions(id)` | `ForeignKeyViolation` |
| `t_session_checkins_session_intent_id_fkey` | `t_session_checkins` | `session_intent_id` | `t_session_intents(id)` | `ForeignKeyViolation` |

## Como Usar

### Em Repositórios

```rust
use crate::infrastructure::repositories::{error::RepositoryError, constraint_mapper};

// Antes (código manual)
.map_err(|error| {
    if let Some(db_err) = error.as_database_error() {
        if let Some(code) = db_err.code() {
            if code == "23505" {
                if let Some(constraint) = db_err.constraint() {
                    match constraint {
                        "t_users_name_key" => {
                            return RepositoryError::UsernameAlreadyTaken(user.name.clone());
                        }
                        "t_users_email_key" => {
                            return RepositoryError::EmailAlreadyTaken(user.email.clone());
                        }
                        _ => return RepositoryError::DatabaseError(error),
                    }
                }
            }
        }
    }
    RepositoryError::DatabaseError(error)
})?

// Depois (usando o mapper)
.map_err(constraint_mapper::map_database_error)?
```

### Exemplo Completo

```rust
use crate::infrastructure::repositories::{error::RepositoryError, constraint_mapper};

#[async_trait::async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, user: &CreateUserCommand) -> Result<User> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let created_user = sqlx::query_as::<_, UserModel>(
            r#"
            INSERT INTO t_users (id, name, nickname, email, password_hash, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(&user.name)
        .bind(&user.nickname)
        .bind(&user.email)
        .bind(&user.password)
        .bind(ERoles::User)
        .bind(now)
        .bind(now)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(Self::map_model_to_entity(created_user))
    }
}
```

## Extração de Valores

O sistema tenta extrair automaticamente os valores dos campos violados das mensagens de erro do PostgreSQL:

```
duplicate key value violates unique constraint "t_users_email_key" 
DETAIL: Key (email)=(john@example.com) already exists.
```

Neste caso, o sistema extrairia `"john@example.com"` e retornaria:
```rust
RepositoryError::EmailAlreadyTaken("john@example.com".to_string())
```

## Adicionando Novas Constraints

Para adicionar suporte a novas constraints:

1. Adicione a nova variante à enum `RepositoryError`
2. Adicione o mapeamento em `map_constraint_violation()`
3. Atualize a documentação

### Exemplo para Nova Constraint

```rust
// 1. Adicionar variante
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    // ... outras variantes ...
    #[error("table name already taken: {0}")]
    TableNameAlreadyTaken(String),
}

// 2. Adicionar mapeamento
pub fn map_constraint_violation(error: &SqlxError, constraint: &str) -> RepositoryError {
    match constraint {
        // ... outros mapeamentos ...
        "t_rpg_tables_title_key" => {
            let title = extract_field_from_error(error, "title").unwrap_or_else(|| "unknown".to_string());
            RepositoryError::TableNameAlreadyTaken(title)
        }
        // ... resto do match ...
    }
}

// 3. Adicionar ao IntoResponse
impl IntoResponse for RepositoryError {
    fn into_response(self) -> Response {
        match self {
            // ... outros casos ...
            Self::TableNameAlreadyTaken(title) => {
                tracing::error!("Table name already taken: {}", title);
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "message": "Table name already taken",
                        "value": title
                    })),
                )
                    .into_response()
            }
            // ... resto do match ...
        }
    }
}
```

## Benefícios

1. **Consistência**: Todos os repositórios usam o mesmo sistema de mapeamento
2. **Manutenibilidade**: Centralizado em um local
3. **Extensibilidade**: Fácil adicionar novas constraints
4. **Legibilidade**: Código mais limpo nos repositórios
5. **Robustez**: Fallback para constraints não reconhecidas

## Testes

Para testar o sistema:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_constraint_mapping() {
        let error_msg = "duplicate key value violates unique constraint \"t_users_name_key\" DETAIL: Key (name)=(john) already exists.";
        // Simular erro do PostgreSQL e verificar mapeamento
    }
}
```
