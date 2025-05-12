use sqlx::postgres::PgPoolOptions;

mod tables;

pub async fn get_one_logbook_entry() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/postgres").await?;

    let row = sqlx::query_as::<_, tables::logbook_entry::LogbookEntry>("select * from logbook_entry").fetch_one(&pool).await?;

    println!("{:?}", row);

    Ok(())
}