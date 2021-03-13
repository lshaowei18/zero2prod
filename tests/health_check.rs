/*
`actix_rt::test` is the testing equivalent of `actix_web::main`.
Spares me from having to specify the `#[test]` attribute.
*/
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app().await;

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

async fn spawn_app() -> String {
    // Port 0 is a special-case at the OS level; it triggers an OS
    // scan for an available port which will then be bound to the application
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener)
        .await
        .expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    let _ = tokio::spawn(server);

    // return the application address to the caller
    format!("http://127.0.0.1:{}", port)
}
