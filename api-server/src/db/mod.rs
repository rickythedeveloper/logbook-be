use sqlx::postgres::PgPoolOptions;

mod tables;
mod pool_utils;

pub async fn get_one_logbook_entry() -> Result<(), sqlx::Error> {
    let pool = pool_utils::get_connection_pool().await;

    let row = sqlx::query_as::<_, tables::logbook_entry::LogbookEntry>("select * from logbook_entry").fetch_one(&pool).await?;

    println!("{:?}", row);

    Ok(())
}