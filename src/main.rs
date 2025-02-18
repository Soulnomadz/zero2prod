use sqlx::PgPool;
use std::net::TcpListener;
use zero::configuration::get_config;
use zero::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let subscriber = get_subscriber("zero".into(), "info".into(), std::io::stdout);
  init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration");
    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let addr = format!("0.0.0.0:{}", config.app_port);
    let listener = TcpListener::bind(addr)?;
    zero::startup::run(listener, connection)?.await
}

