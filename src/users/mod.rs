pub mod models;
mod routes;
mod schemas;
mod services;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(routes::delete_user_handler)
        .service(routes::login_user_handler)
        .service(routes::register_user_handler);
}
