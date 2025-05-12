mod db;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::get_one_logbook_entry().await.unwrap();

    server::start_server().await;

    Ok(())
}
