use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = the_news_letter::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn test_health_check() {

    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}

#[tokio::test]
async fn if_return_is_400_data_missing() {

    let address = spawn_app();

    let client = reqwest::Client::new();
    let test_cases = vec![("email=fakeemail","email but no name"),("name=fakename", "name but no email"),("","no email or name")];

    for (invalid_body, error_message) in test_cases {
        let response = client.post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Falied to send post request");

        assert_eq!(400, response.status().as_u16(), "The API did not fail with 400 Bad Request when the payload was {}.",
                   error_message);
    }
}

#[tokio::test]
async fn if_return_is_200_ok_from_subscriber() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=Yashas&email=yashasnadigsyn%40proton.me";
    let response = client.post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Falied to send post request");

    assert_eq!(200,  response.status().as_u16());
}
