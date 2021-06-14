use std::net::TcpListener;

use rust_zero2prod::configuration;
use rust_zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("Listening on http://{}", address);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
