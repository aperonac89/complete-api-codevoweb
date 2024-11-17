mod config;
mod db;
mod dtos;
mod errors;
mod extractors;
mod models;
mod scopes;
mod utils;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use config::Config;
use db::DBClient;
use dotenv::dotenv;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix-web=info")
    }

    dotenv().ok();
    env_logger::init();

    let config = Config::init();
    let pool = PgPoolOptions::new()
        .max_connections(30)
        .connect(&config.database_url)
        .await?;

    let db_client = DBClient::new(pool);
    let app_state: AppState = AppState {
        env: config.clone(),
        db_client,
    };

    println!("Server is running on http://localhost:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(health_check)
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await;

    Ok(())
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "success", "message": "up and running"}))
}
