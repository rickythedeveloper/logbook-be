use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgSeverity::Log;
use sqlx::types::uuid::Uuid;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[derive(sqlx::FromRow)]
struct LogbookEntry {
    id: Uuid
}

async fn test_sql() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/postgres").await?;

    let row = sqlx::query_as::<_, LogbookEntry>("select * from logbook_entry").fetch_one(&pool).await?;

    println!("{}", row.id);

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    test_sql().await.unwrap();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}