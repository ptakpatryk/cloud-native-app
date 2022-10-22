use std::net::TcpListener;

use zero2prod::startup;
use zero2prod::configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if can't read config file
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind the port");
    startup::run(listener)?.await
}
