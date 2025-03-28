use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::types::Uuid;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use zero::configuration::{get_config, DatabaseSettings};
use zero::telemetry::{get_subscriber, init_subscriber};

use std::net::TcpListener;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_succeeds() {
    let addr = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", addr.address))
        .send()
        .await
        .expect("Failed to execute request!");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_config().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = config_database(&configuration.database).await;

    let server =
        zero::startup::run(listener, connection_pool.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    //format!("http://127.0.0.1:{port}")

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn config_database(config: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(&config.connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to database");

    // create database
    connection
        .execute(format!(r#"create database "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    // database migration
    let pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    pool
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Ursula&email=not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let resp = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        //let saved = sqlx::query!("select email,name from subscriptions")
        //    .fetch_one(&app.db_pool)
        //    .await
        //    .expect("Failed to fetch saved subscription.");

        assert_eq!(
            400,
            resp.status().as_u16(),
            "The API dit not return 400 when payload was {description}"
        );
    }
}

#[tokio::test]
async fn subscribe_return_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (body, err_msg) in test_cases {
        let resp = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            resp.status().as_u16(),
            "The API dit not fail with 400 when the payload was {err_msg}",
        )
    }
}

