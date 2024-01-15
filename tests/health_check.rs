use std::net::TcpListener;
use sqlx::{PgPool};
use the_news_letter::configuration::get_configuration;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration!");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Cannot connect to database!");
    let server = the_news_letter::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }

}

#[tokio::test]
async fn test_health_check() {

    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
// Arrange
    let app_address = spawn_app().await;
    let configuration = get_configuration().expect("Cannot get configuration!");
    let connection_string = configuration.database.connection_string();
    //let mut connection = PgConnection::connect(&connection_string).await.expect("Failed to connect to postgres");
    let client = reqwest::Client::new();
// Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions").fetch_one(&app_address.db_pool).await.expect("FAILED TO FETCH DETAILES");

    assert_eq!(saved.email, "rsula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}


#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
// Arrange
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
    for (invalid_body, error_message) in test_cases {
// Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
// Assert
        assert_eq!(
            400,
            response.status().as_u16(),
// Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}