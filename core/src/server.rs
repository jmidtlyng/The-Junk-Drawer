use crate::router;
use actix_web::{App, HttpServer, dev::Server, web};
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;
use std::sync::Mutex;

pub struct JunkDrawer {
    pub junk: Mutex<i32>
}

pub fn serve(listener: TcpListener) -> Result<Server, std::io::Error> {
    let entities = web::Data::new(JunkDrawer {
        junk: Mutex::new(0),
    });
    
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