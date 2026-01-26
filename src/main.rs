use actix_web::{App, HttpServer};

mod db;
mod error;
mod models;
mod prelude;
mod routes;
mod schemas;
mod services;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    db::init_db()
        .await
        .map_err(|e| format!("Faild to init database: {}", e))?;

    db::init_tables()
        .await
        .map_err(|e| format!("Faild to init tables: {}", e))?;

    tracing::info!("Starting server on http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .service(routes::users::register_user_handler)
            .service(routes::users::login_user_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
