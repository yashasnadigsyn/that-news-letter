pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use routes::health_check;
use routes::subscribe;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!("{:#?}", &listener);
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();

    Ok(server)
}
