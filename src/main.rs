use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if can't read config file
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the database.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind the port");
    startup::run(listener, connection)?.await
}
