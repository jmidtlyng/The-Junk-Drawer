use crate::router;
use actix_web::{App, HttpServer, dev::Server};
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;
use std::sync::Mutex;

struct Junk {
    
}

pub fn serve(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .configure(router::config)
    })
    .listen(listener)?
    .run();

    Ok(server)
}