use std::net::TcpListener;
use sqlx::{PgConnection, Connection};
use zero2prod::configuration::{self, get_configuration};

fn spawn_app() -> String {
    // port 0 checks what port is available and selects randomly between them
    let listener = TcpListener::bind("127.0.0.1:0")
                    .expect("Failed to bind random port.");
    // getting the port we're assigned to
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task, spawn takes a future and hands
    // it to runtime without waiting for its completion
    let _ = tokio::spawn(server);
    // return application address to the caller
    format!("http://127.0.0.1:{}", port)

}


#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address)) // responds to GET?
        .send()                                    // responds to /health_check
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());       // returns a 200?
    assert_eq!(Some(0), response.content_length());// has no body?
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {

    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
                   .post(&format!("{}/subscriptions", &app_address))
                   .header("Content-Type", "application/x-www-form-urlencoded")
                   .body(body)
                   .send()
                   .await
                   .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    // Cool tests!
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
                .fetch_one(&mut connection)
                .await
                .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le_guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let request = client
                      .post(&format!("{}/subscriptions", &app_address))
                      .header("Content-Type", "application/x-www-form-urlencoded")
                      .body(invalid_body)
                      .send()
                      .await
                      .expect("Failed to execute the request.");

        assert_eq!(400, request.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message)
    }

}
