use crate::routes::html;
use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;
use actix_web::dev::Server;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .configure(html::config)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
