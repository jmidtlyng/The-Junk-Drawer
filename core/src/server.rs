use crate::router;
use crate::models::{JunkDrawer, Junk};
use actix_web::{App, HttpServer, dev::Server, web};
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;
use std::sync::Mutex;


pub fn serve(listener: TcpListener) -> Result<Server, std::io::Error> {
    let entities = web::Data::new( models::JunkDrawer );
    
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .app_data(entities.clone())
            .configure(router::config)
    })
    .listen(listener)?
    .run();

    Ok(server)
}