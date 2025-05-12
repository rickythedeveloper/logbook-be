use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn get_connection_pool() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap();

    pool
}
