use actix_web::{App, HttpServer};
mod database;
mod handlers;
mod models;
mod schema;
use handlers::handlers::*;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(scoreboard))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
