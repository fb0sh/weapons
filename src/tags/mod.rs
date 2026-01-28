pub mod models;
mod routes;
mod schemas;
mod services;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(routes::get_tags)
        .service(routes::create_tag)
        .service(routes::update_tag)
        .service(routes::delete_tag);
}
