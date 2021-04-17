#[macro_use]
extern crate log;

use std::env;

use actix_web::{App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> Result<()> {
    // add env file to app, and start logging.
    dotenv().ok();
    env_logger::init();

    // setup database pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await?;

    // start server
    info!("Starting server at: 127.0.0.1:8080");
    HttpServer::new(move || App::new().data(db_pool.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}

