mod _core;
mod _indexes;
mod prelude;

mod categories;
mod items;
mod tags;
mod users;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    _core::init_data_dir()
        .await
        .map_err(|e| format!("Faild to init data dir: {}", e))?;

    let _guard = _core::init_tracing();

    _core::init_db()
        .await
        .map_err(|e| format!("Faild to init database: {}", e))?;

    _core::init_tables()
        .await
        .map_err(|e| format!("Faild to init tables: {}", e))?;

    tracing::info!("Starting server on http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            // /users
            .configure(users::config)
            // /categories
            .configure(categories::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
