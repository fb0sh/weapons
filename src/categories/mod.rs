pub mod models;
pub mod routes;
pub mod schemas;
mod services;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(routes::get_categories)
        .service(routes::create_category);
}
