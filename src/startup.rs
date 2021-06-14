use crate::routes;
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route(
                "/health-check",
                web::get().to(routes::health_check::health_check),
            )
            .route(
                "/subscriptions",
                web::post().to(routes::subscriptions::subscribe),
            )
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}
