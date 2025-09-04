pub struct PostgresTableMemberRepository {
    pool: PgPool,
}

impl PostgresTableMemberRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
