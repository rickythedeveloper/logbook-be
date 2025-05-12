use sqlx::postgres::PgPoolOptions;
use std::any::Any;

mod pool_utils;
mod tables;

pub async fn get_one_logbook_entry() -> Result<(), sqlx::Error> {
    let pool = pool_utils::get_connection_pool().await;

    let row =
        sqlx::query_as::<_, tables::logbook_entry::LogbookEntry>("select * from logbook_entry_2")
            .fetch_one(&pool)
            .await;

    match row {
        Ok(row) => println!("{:?}", row),
        Err(e) => match e {
            sqlx::Error::RowNotFound => println!("No row found"),
            sqlx::Error::Database(db_err) => println!("Database error: {:?}", db_err),
            _ => println!("Error: {:?}", e),
        },
    }

    Ok(())
}
