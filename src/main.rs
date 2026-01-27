mod _core;
mod _indexes;
mod prelude;

mod categories;
mod items;
mod tags;
mod users;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    _core::init_db()
        .await
        .map_err(|e| format!("Faild to init database: {}", e))?;

    _core::init_tables()
        .await
        .map_err(|e| format!("Faild to init tables: {}", e))?;

    tracing::info!("Starting server on http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .service(users::routes::login_user_handler)
            .service(users::routes::register_user_handler)
            .service(categories::routes::get_categories)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
