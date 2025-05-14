mod db;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = db::DatabaseImpl {};

    server::start_server(database).await;

    Ok(())
}
