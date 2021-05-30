/*
`actix_rt::test` is the testing equivalent of `actix_web::main`.
Spares me from having to specify the `#[test]` attribute.
*/
use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::configuration::get_configuration;

use sqlx::Row;

const FORM_HEADER: &str = "application/x-www-form-urlencoded";

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();

    // `reqwest` to perform HTTP requests against our application
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", FORM_HEADER)
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let saved = sqlx::query("SELECT email, name FROM subscriptions")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch saved subscription.");

    let email: String = saved.get("email");
    let name: String = saved.get("name");
    assert_eq!(email, "ursula_le_guin@gmail.com");
    assert_eq!(name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", FORM_HEADER)
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

fn spawn_app() -> String {
    // Port 0 is a special-case at the OS level; it triggers an OS
    // scan for an available port which will then be bound to the application
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    let _ = tokio::spawn(server);

    // return the application address to the caller
    format!("http://127.0.0.1:{}", port)
}
