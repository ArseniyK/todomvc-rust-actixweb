#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Logger};
use actix_cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod api;
mod db;
mod models;
mod schema;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "0.0.0.0:8000";

    println!("Starting server at: {}", &bind);

    let app = move || {
        println!("Constructing the App");

        App::new()
            .data(pool.clone())
            .wrap(
                Cors::default()
            )
            .wrap(Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(api::index))
                    .route(web::post().to(api::create))
                    .route(web::delete().to(api::delete_all))
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(api::get))
                    .route(web::patch().to(api::update))
                    .route(web::delete().to(api::delete))
            )
    };

    HttpServer::new(app).bind(bind)?.run().await
}