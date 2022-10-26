use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration;
use zero2prod::startup;
use zero2prod::telemetry::get_subscriber;
use zero2prod::telemetry::init_subscriber;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the database.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind the port");
    startup::run(listener, connection_pool)?.await
}
