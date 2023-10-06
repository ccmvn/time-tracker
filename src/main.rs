use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use lazy_static::lazy_static;
use log::LevelFilter;
use sqlx::SqlitePool;

use crate::database::index::DatabaseConnection;
use crate::routes::main_config;
use crate::templates::tera_config;
use crate::utils::config::Config;

mod database;
mod middleware;
mod pages;
mod utils;
mod routes;
mod templates;

lazy_static! {
    static ref CONFIG: Config = Config::new().unwrap();
}

fn init_environment() {
    // Initialize logger
    env_logger::Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();

    // Load dotenv
    dotenv::dotenv().ok();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    init_environment();

    // Setup the database connection pool
    let database_url = "sqlite:time-tracker.db";
    let pool = SqlitePool::connect(&database_url).await.unwrap();
    let database = DatabaseConnection { pool };

    // Actix Web Server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(database.clone()))
            .configure(tera_config)
            .configure(main_config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
