// use views;
/*
use core::config::get_config;
use std::net::TcpListener;
use core::runner::run;
    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        let config = get_config().expect("Failed to read configuration.");
        let address = format!("127.0.0.1:{}", config.application_port);
        let listener = TcpListener::bind(address)?;
        run(listener)?.await
    }
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
*/
fn main() {
    println!("yo");
}