use crate::Result;
use crate::domain::table::commands::{
    CreateTableCommand, DeleteTableCommand, GetTableCommand, UpdateTableCommand,
};
use crate::domain::table::entity::Table;
use crate::domain::table::table_repository::TableRepository as TableRepositoryTrait;
use crate::domain::utils::update::Update;
use crate::infrastructure::entities::enums::ETableVisibility;
use crate::infrastructure::entities::t_rpg_tables::Model as TableModel;
use crate::infrastructure::prelude::RepositoryError;
use crate::infrastructure::repositories::constraint_mapper;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresTableRepository {
    pool: Arc<PgPool>,
}

impl PostgresTableRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TableRepositoryTrait for PostgresTableRepository {
    async fn create(&self, command: &CreateTableCommand) -> Result<Table> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let visibility: ETableVisibility = command.visibility.into();

        let created_table = sqlx::query_as!(
            TableModel,
            r#"INSERT INTO t_rpg_tables 
                (id, 
                gm_id, 
                title, 
                visibility, 
                description, 
                game_system_id, 
                player_slots, 
                created_at, 
                updated_at)
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                id,
                gm_id,
                title,
                visibility as "visibility: ETableVisibility",
                description,
                game_system_id,
                player_slots,
                created_at,
                updated_at
            "#,
            id,
            command.gm_id,
            command.title,
            visibility as _,
            command.description,
            command.game_system_id,
            command.player_slots as i32,
            now,
            now
        )
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(created_table.into())
    }

    async fn update(&self, command: &UpdateTableCommand) -> Result<Table> {
        let now = Utc::now();

        let mut builder = sqlx::QueryBuilder::new("UPDATE t_rpg_tables SET ");

        // Usa `separated` para gerenciar as vírgulas na cláusula SET.
        let mut separated = builder.separated(", ");

        if let Update::Change(title) = &command.title {
            separated.push("title = ");
            separated.push_bind_unseparated(title);
        }

        if let Update::Change(description) = &command.description {
            separated.push("description = ");
            separated.push_bind_unseparated(description);
        }

        if let Update::Change(visibility) = &command.visibility {
            separated.push("visibility = ");
            separated.push_bind_unseparated(ETableVisibility::from(*visibility));
        }

        if let Update::Change(player_slots) = &command.player_slots {
            separated.push("player_slots = ");
            separated.push_bind_unseparated(*player_slots as i32);
        }

        if let Update::Change(game_system_id) = &command.game_system_id {
            separated.push("game_system_id = ");
            separated.push_bind_unseparated(game_system_id);
        }

        // A primeira chamada a `push` não adiciona vírgula.
        // As subsequentes sim.
        separated.push("updated_at = ");
        separated.push_bind_unseparated(now);

        // Adiciona a cláusula WHERE após a SET.
        builder.push(" WHERE id = ");
        builder.push_bind(command.id);

        // Adiciona a cláusula RETURNING corretamente.
        builder.push(
            r#" RETURNING 
                id, 
                gm_id, 
                title, 
                visibility, -- CORRIGIDO
                description, 
                game_system_id,
                player_slots,
                created_at,
                updated_at"#,
        );

        // Executa a query construída com todos os seus parâmetros.
        let updated_table = builder
            .build_query_as::<TableModel>()
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(updated_table.into())
    }

    async fn delete(&self, command: &DeleteTableCommand) -> Result<Table> {
        let table = sqlx::query_as!(
            TableModel,
            r#"DELETE FROM t_rpg_tables 
                WHERE id = $1
                RETURNING
                    id,
                    gm_id,
                    title,
                    visibility as "visibility: ETableVisibility",
                    description,
                    game_system_id,
                    player_slots,
                    created_at,
                    updated_at
            "#,
            command.id
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        match table {
            Some(table) => Ok(table.into()),
            None => {
                return Err(RepositoryError::TableNotFound.into());
            }
        }
    }

    async fn get(&self, command: &GetTableCommand) -> Result<Vec<Table>> {
        let mut builder = sqlx::QueryBuilder::new(
            r#"SELECT 
                id, 
                gm_id, 
                title, 
                visibility, 
                description, 
                game_system_id, 
                player_slots, 
                created_at, 
                updated_at 
            FROM t_rpg_tables"#,
        );

        let mut has_where = false;
        let mut push_filter_separator = |b: &mut sqlx::QueryBuilder<'_, sqlx::Postgres>| {
            if !has_where {
                b.push(" WHERE ");
                has_where = true;
            } else {
                b.push(" AND ");
            }
        };

        if let Some(id) = &command.filters.id {
            push_filter_separator(&mut builder);
            builder.push("id = ");
            builder.push_bind(id);
        }

        if let Some(gm_id) = &command.filters.gm_id {
            push_filter_separator(&mut builder);
            builder.push("gm_id = ");
            builder.push_bind(gm_id);
        }

        if let Some(title) = &command.filters.title {
            push_filter_separator(&mut builder);
            builder.push("title = ");
            builder.push_bind(title);
        }

        if let Some(visibility) = &command.filters.visibility {
            push_filter_separator(&mut builder);
            builder.push("visibility = ");
            builder.push_bind(ETableVisibility::from(*visibility));
        }

        if let Some(description) = &command.filters.description {
            push_filter_separator(&mut builder);
            builder.push("description = ");
            builder.push_bind(description);
        }

        if let Some(game_system_id) = &command.filters.game_system_id {
            push_filter_separator(&mut builder);
            builder.push("game_system_id = ");
            builder.push_bind(game_system_id);
        }

        if let Some(player_slots) = &command.filters.player_slots {
            push_filter_separator(&mut builder);
            builder.push("player_slots = ");
            builder.push_bind(*player_slots as i32);
        }

        if let Some(created_at) = &command.filters.created_at {
            push_filter_separator(&mut builder);
            builder.push("created_at = ");
            builder.push_bind(created_at);
        }

        if let Some(updated_at) = &command.filters.updated_at {
            push_filter_separator(&mut builder);
            builder.push("updated_at = ");
            builder.push_bind(updated_at);
        }

        let page = command.pagination.limit();
        let offset = command.pagination.offset();

        builder.push(" LIMIT ");
        builder.push_bind(page as i64);

        builder.push(" OFFSET ");
        builder.push_bind(offset as i64);

        let tables = builder
            .build_query_as::<TableModel>()
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|m| m.into()).collect())
    }

    async fn find_by_id(&self, table_id: &Uuid) -> Result<Table> {
        let table = sqlx::query_as!(
            TableModel,
            r#"SELECT 
                id, 
                gm_id, 
                title, 
                visibility as "visibility: ETableVisibility", 
                description, 
                game_system_id, 
                player_slots, 
                created_at, 
                updated_at 
            FROM t_rpg_tables 
            WHERE id = $1"#,
            table_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        match table {
            Some(table) => Ok(table.into()),
            None => Err(RepositoryError::TableNotFound.into()),
        }
    }

    async fn find_by_gm_id(&self, gm_id: &Uuid) -> Result<Vec<Table>> {
        let tables = sqlx::query_as!(
            TableModel,
            r#"SELECT 
                id, 
                gm_id, 
                title, 
                visibility as "visibility: ETableVisibility", 
                description, 
                game_system_id, 
                player_slots, 
                created_at, 
                updated_at 
            FROM t_rpg_tables
            WHERE gm_id = $1"#,
            gm_id
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(constraint_mapper::map_database_error)?;

        Ok(tables.into_iter().map(|m| m.into()).collect())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Error;
    use crate::domain::game_system::GameSystemRepository;
    use crate::domain::table::entity::Visibility;
    use crate::domain::table::search_filters::TableFilters;
    use crate::domain::user::UserRepository;
    use crate::domain::user::commands::CreateUserCommand;
    use crate::domain::utils::pagination::Pagination;
    use crate::infrastructure::prelude::PostgresGameSystemRepository;
    use crate::infrastructure::repositories::user::PostgresUserRepository;
    use sqlx::PgPool;
    use std::sync::Arc;
    use uuid::Uuid;

    fn create_test_table_data(
        gm_id: Uuid,
        title: &str,
        description: &str,
        visibility: Visibility,
        player_slots: u32,
        game_system_id: Uuid,
    ) -> CreateTableCommand {
        CreateTableCommand {
            gm_id,
            title: title.to_string(),
            description: description.to_string(),
            visibility,
            player_slots,
            game_system_id,
        }
    }

    async fn create_test_user(pool: &PgPool) -> Uuid {
        let repo = PostgresUserRepository::new(Arc::new(pool.clone()));

        let username = format!("testuser{}", Uuid::new_v4());
        let display_name = format!("Test User {}", Uuid::new_v4());
        let email = format!("testuser{}@example.com", Uuid::new_v4());

        let user_data = CreateUserCommand {
            username: username.clone(),
            display_name: display_name.clone(),
            email: email.clone(),
            password: "password123".to_string(),
        };
        let user = repo.create(&user_data).await.unwrap();
        user.id
    }

    async fn create_test_game_system(pool: &PgPool) -> Uuid {
        let repo = PostgresGameSystemRepository::new(Arc::new(pool.clone()));
        let game_system_name = format!("Test Game System {}", Uuid::new_v4());
        let game_system = repo.create(&game_system_name).await.unwrap();
        game_system.id
    }

    async fn setup_test_data(pool: &PgPool) -> (Uuid, Uuid) {
        let gm_id = create_test_user(pool).await;
        let game_system_id = create_test_game_system(pool).await;
        (gm_id, game_system_id)
    }

    #[sqlx::test]
    async fn test_create_table_success(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let table_data = create_test_table_data(
            gm_id,
            "Test Table",
            "A test table for RPG",
            Visibility::Public,
            5,
            game_system_id,
        );

        let result = repo.create(&table_data).await;

        match result {
            Ok(table) => {
                assert_eq!(table.title, "Test Table");
                assert_eq!(table.description, "A test table for RPG");
                assert_eq!(table.visibility, Visibility::Public);
                assert_eq!(table.player_slots, 5);
                assert_eq!(table.gm_id, gm_id);
                assert_eq!(table.game_system_id, game_system_id);
                assert!(table.id != uuid::Uuid::nil());
            }
            Err(e) => {
                panic!("Unexpected error: {e:?}");
            }
        }
    }

    #[sqlx::test]
    async fn test_create_table_private_visibility(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let table_data = create_test_table_data(
            gm_id,
            "Private Table",
            "A private test table",
            Visibility::Private,
            3,
            game_system_id,
        );

        let result = repo.create(&table_data).await;

        match result {
            Ok(table) => {
                assert_eq!(table.title, "Private Table");
                assert_eq!(table.visibility, Visibility::Private);
                assert_eq!(table.player_slots, 3);
            }
            Err(e) => {
                panic!("Unexpected error: {e:?}");
            }
        }
    }

    #[sqlx::test]
    async fn test_find_by_id(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let table_data = create_test_table_data(
            gm_id,
            "Test Table",
            "A test table for RPG",
            Visibility::Public,
            5,
            game_system_id,
        );

        let created_table = repo.create(&table_data).await.unwrap();
        let found_table = repo.find_by_id(&created_table.id).await;

        assert!(found_table.is_ok());
        let found_table = found_table.unwrap();
        assert_eq!(found_table.id, created_table.id);
        assert_eq!(found_table.title, "Test Table");
        assert_eq!(found_table.description, "A test table for RPG");
        assert_eq!(found_table.visibility, Visibility::Public);
        assert_eq!(found_table.player_slots, 5);
    }

    #[sqlx::test]
    async fn test_find_by_id_not_found(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));

        let random_id = Uuid::new_v4();
        let result = repo.find_by_id(&random_id).await;

        assert!(result.is_err());

        if let Err(Error::Repository(RepositoryError::TableNotFound)) = result {
            // Expected error
        } else {
            panic!("Expected TableNotFound error");
        }
    }

    #[sqlx::test]
    async fn test_find_by_gm_id(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let table_data1 = create_test_table_data(
            gm_id,
            "Table 1",
            "First table",
            Visibility::Public,
            5,
            game_system_id,
        );

        let table_data2 = create_test_table_data(
            gm_id,
            "Table 2",
            "Second table",
            Visibility::Private,
            3,
            game_system_id,
        );

        repo.create(&table_data1).await.unwrap();
        repo.create(&table_data2).await.unwrap();

        let found_tables = repo.find_by_gm_id(&gm_id).await.unwrap();
        assert_eq!(found_tables.len(), 2);

        let titles: Vec<String> = found_tables.iter().map(|t| t.title.clone()).collect();
        assert!(titles.contains(&"Table 1".to_string()));
        assert!(titles.contains(&"Table 2".to_string()));
    }

    #[sqlx::test]
    async fn test_find_by_gm_id_empty(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool));
        let random_gm_id = Uuid::new_v4();

        let found_tables = repo.find_by_gm_id(&random_gm_id).await.unwrap();
        assert_eq!(found_tables.len(), 0);
    }

    #[sqlx::test]
    async fn test_get_all_tables(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id1, game_system_id) = setup_test_data(&pool).await;
        let gm_id2 = create_test_user(&pool).await;

        let table_data1 = create_test_table_data(
            gm_id1,
            "Table 1",
            "First table",
            Visibility::Public,
            5,
            game_system_id,
        );

        let table_data2 = create_test_table_data(
            gm_id2,
            "Table 2",
            "Second table",
            Visibility::Private,
            3,
            game_system_id,
        );

        repo.create(&table_data1).await.unwrap();
        repo.create(&table_data2).await.unwrap();

        let get_command = GetTableCommand::default();
        let all_tables = repo.get(&get_command).await.unwrap();
        assert_eq!(all_tables.len(), 2);

        let titles: Vec<String> = all_tables.iter().map(|t| t.title.clone()).collect();
        assert!(titles.contains(&"Table 1".to_string()));
        assert!(titles.contains(&"Table 2".to_string()));
    }

    #[sqlx::test]
    async fn test_get_tables_with_filters(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let table_data1 = create_test_table_data(
            gm_id,
            "Public Table",
            "Public table",
            Visibility::Public,
            5,
            game_system_id,
        );

        let table_data2 = create_test_table_data(
            gm_id,
            "Private Table",
            "Private table",
            Visibility::Private,
            3,
            game_system_id,
        );

        repo.create(&table_data1).await.unwrap();
        repo.create(&table_data2).await.unwrap();

        let filters = TableFilters::default().with_visibility(Visibility::Public);
        let get_command = GetTableCommand::default().with_filters(filters);
        let public_tables = repo.get(&get_command).await.unwrap();

        assert_eq!(public_tables.len(), 1);
        assert_eq!(public_tables[0].title, "Public Table");
        assert_eq!(public_tables[0].visibility, Visibility::Public);
    }

    #[sqlx::test]
    async fn test_update_table_title(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let table_data = create_test_table_data(
            gm_id,
            "Original Title",
            "A test table for RPG",
            Visibility::Public,
            5,
            game_system_id,
        );

        let created_table = repo.create(&table_data).await.unwrap();

        let update_data = UpdateTableCommand {
            id: created_table.id,
            title: Update::Change("Updated Title".to_string()),
            description: Update::Keep,
            visibility: Update::Keep,
            player_slots: Update::Keep,
            game_system_id: Update::Keep,
        };

        repo.update(&update_data).await.unwrap();

        let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
        assert_eq!(updated_table.title, "Updated Title");
        assert_eq!(updated_table.description, "A test table for RPG");
        assert_eq!(updated_table.visibility, Visibility::Public);
    }

    #[sqlx::test]
    async fn test_update_table_description(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let table_data = create_test_table_data(
            gm_id,
            "Test Table",
            "Original description",
            Visibility::Public,
            5,
            game_system_id,
        );

        let created_table = repo.create(&table_data).await.unwrap();

        let update_data = UpdateTableCommand {
            id: created_table.id,
            title: Update::Keep,
            description: Update::Change("Updated description".to_string()),
            visibility: Update::Keep,
            player_slots: Update::Keep,
            game_system_id: Update::Keep,
        };

        repo.update(&update_data).await.unwrap();

        let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
        assert_eq!(updated_table.title, "Test Table"); // Not changed
        assert_eq!(updated_table.description, "Updated description");
    }

    #[sqlx::test]
    async fn test_update_table_visibility(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let table_data = create_test_table_data(
            gm_id,
            "Test Table",
            "A test table",
            Visibility::Public,
            5,
            game_system_id,
        );

        let created_table = repo.create(&table_data).await.unwrap();

        let update_data = UpdateTableCommand {
            id: created_table.id,
            title: Update::Keep,
            description: Update::Keep,
            visibility: Update::Change(Visibility::Private),
            player_slots: Update::Keep,
            game_system_id: Update::Keep,
        };

        repo.update(&update_data).await.unwrap();

        let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
        assert_eq!(updated_table.visibility, Visibility::Private);
    }

    #[sqlx::test]
    async fn test_update_table_player_slots(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let table_data = create_test_table_data(
            gm_id,
            "Test Table",
            "A test table",
            Visibility::Public,
            5,
            game_system_id,
        );

        let created_table = repo.create(&table_data).await.unwrap();

        let update_data = UpdateTableCommand {
            id: created_table.id,
            title: Update::Keep,
            description: Update::Keep,
            visibility: Update::Keep,
            player_slots: Update::Change(8),
            game_system_id: Update::Keep,
        };

        repo.update(&update_data).await.unwrap();

        let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
        assert_eq!(updated_table.player_slots, 8);
    }

    #[sqlx::test]
    async fn test_update_table_game_system_id(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id1) = setup_test_data(&pool).await;
        let (_, game_system_id2) = setup_test_data(&pool).await;

        let table_data = create_test_table_data(
            gm_id,
            "Test Table",
            "A test table",
            Visibility::Public,
            5,
            game_system_id1,
        );

        let created_table = repo.create(&table_data).await.unwrap();

        let update_data = UpdateTableCommand {
            id: created_table.id,
            game_system_id: Update::Change(game_system_id2),
            ..Default::default()
        };

        repo.update(&update_data).await.unwrap();

        let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
        assert_eq!(updated_table.game_system_id, game_system_id2);
    }

    #[sqlx::test]
    async fn test_update_table_multiple_fields(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let table_data = create_test_table_data(
            gm_id,
            "Original Title",
            "Original description",
            Visibility::Public,
            5,
            game_system_id,
        );

        let created_table = repo.create(&table_data).await.unwrap();

        let update_data = UpdateTableCommand {
            id: created_table.id,
            title: Update::Change("New Title".to_string()),
            description: Update::Change("New description".to_string()),
            visibility: Update::Change(Visibility::Private),
            player_slots: Update::Change(7),
            ..Default::default()
        };

        repo.update(&update_data).await.unwrap();

        let updated_table = repo.find_by_id(&created_table.id).await.unwrap();
        assert_eq!(updated_table.title, "New Title");
        assert_eq!(updated_table.description, "New description");
        assert_eq!(updated_table.visibility, Visibility::Private);
        assert_eq!(updated_table.player_slots, 7);
    }

    #[sqlx::test]
    async fn test_update_table_not_found(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool));

        let random_id = Uuid::new_v4();
        let update_data = UpdateTableCommand {
            id: random_id,
            title: Update::Change("New Title".to_string()),
            description: Update::Keep,
            visibility: Update::Keep,
            player_slots: Update::Keep,
            game_system_id: Update::Keep,
        };

        let result = repo.update(&update_data).await;

        match result {
            Err(Error::Repository(RepositoryError::TableNotFound)) => (),
            _ => panic!("Unexpected error: {result:?}"),
        }
    }

    #[sqlx::test]
    async fn test_delete_table(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool.clone()).await;

        let table_data = create_test_table_data(
            gm_id,
            "Test Table",
            "A test table for RPG",
            Visibility::Public,
            5,
            game_system_id,
        );

        let created_table = repo
            .create(&table_data)
            .await
            .expect("Failed to create table");

        let delete_command = DeleteTableCommand {
            id: created_table.id,
            gm_id: created_table.gm_id,
        };

        let deleted_table = repo
            .delete(&delete_command)
            .await
            .expect("Failed to delete table");

        assert_eq!(deleted_table.id, created_table.id);
        assert_eq!(deleted_table.title, "Test Table");
    }

    #[sqlx::test]
    async fn test_delete_table_not_found(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool));

        let random_id = Uuid::new_v4();
        let random_gm_id = Uuid::new_v4();
        let delete_command = DeleteTableCommand {
            id: random_id,
            gm_id: random_gm_id,
        };

        let result = repo.delete(&delete_command).await;

        match result {
            Err(Error::Repository(RepositoryError::TableNotFound)) => (),
            _ => panic!("Unexpected error: {result:?}"),
        }
    }

    #[sqlx::test]
    async fn test_concurrent_table_operations(pool: PgPool) {
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        let handles: Vec<_> = (0..5)
            .map(|i| {
                let pool = pool.clone();
                let table_data = create_test_table_data(
                    gm_id,
                    &format!("Table {i}"),
                    &format!("Description for table {i}"),
                    if i % 2 == 0 {
                        Visibility::Public
                    } else {
                        Visibility::Private
                    },
                    3 + i as u32,
                    game_system_id,
                );
                tokio::spawn(async move {
                    let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
                    repo.create(&table_data)
                        .await
                        .expect("Failed to create table")
                })
            })
            .collect();

        let results: Vec<_> = futures::future::join_all(handles).await;

        for result in results {
            assert!(result.is_ok());
        }

        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let get_command = GetTableCommand::default();
        let all_tables = repo
            .get(&get_command)
            .await
            .expect("Failed to get all tables");
        assert_eq!(all_tables.len(), 5);
    }

    #[sqlx::test]
    async fn test_pagination(pool: PgPool) {
        let repo = PostgresTableRepository::new(Arc::new(pool.clone()));
        let (gm_id, game_system_id) = setup_test_data(&pool).await;

        // Create 25 tables
        for i in 0..25 {
            let table_data = create_test_table_data(
                gm_id,
                &format!("Table {i}"),
                &format!("Description {i}"),
                Visibility::Public,
                5,
                game_system_id,
            );
            repo.create(&table_data)
                .await
                .expect("Failed to create table");
        }

        // Test first page (default page size is 20)
        let pagination = Pagination::default();
        let get_command = GetTableCommand::default().with_pagination(pagination);
        let first_page = repo
            .get(&get_command)
            .await
            .expect("Failed to get first page");
        assert_eq!(first_page.len(), 20);

        // Test second page
        let pagination = Pagination::default().with_page(2);
        let get_command = GetTableCommand::default().with_pagination(pagination);
        let second_page = repo
            .get(&get_command)
            .await
            .expect("Failed to get second page");
        assert_eq!(second_page.len(), 5);

        // Test custom page size
        let pagination = Pagination::default().with_page_size(10);
        let get_command = GetTableCommand::default().with_pagination(pagination);
        let custom_page = repo
            .get(&get_command)
            .await
            .expect("Failed to get custom page");
        assert_eq!(custom_page.len(), 10);
    }
}
