use sqlx::PgPool;
use std::net::TcpListener;
use zero::configuration::get_config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration");

    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");

    //let addr = TcpListener::bind("0.0.0.0:8000").expect("Failed to bind port 8000");
    let addr = format!("0.0.0.0:{}", config.app_port);
    let listener = TcpListener::bind(addr)?;
    zero::startup::run(listener, connection)?.await
}
