use crate::routes;
use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;
use actix_web::dev::Server;
use std::net::TcpListener;

pub fn serve(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .configure(routes::config)
    })
    .listen(listener)?
    .run();

    Ok(server)
}