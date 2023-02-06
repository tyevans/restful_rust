#[macro_use]
extern crate lazy_static;

mod common;
mod auth;
mod templates;
mod blog;
mod database;
mod schema;

use actix_web::{App, HttpServer, web};

use crate::blog::views::blog_route_config;
use crate::auth::views::auth_route_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").service(
                web::scope("/auth")
                    .configure(auth_route_config)
            ).service(
                web::scope("/blog")
                    .configure(blog_route_config)
            )
        )
    }).bind(("0.0.0.0", 8080))?.run().await
}
