mod common;

use common::spawn_app;
use reqwest::StatusCode;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=alexander%20gogas&email=alexander.gogas%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(StatusCode::OK, response.status());
}

#[cfg(feature = "actix")]
const BAD_FORM_STATUS: StatusCode = StatusCode::BAD_REQUEST;
#[cfg(feature = "axum")]
const BAD_FORM_STATUS: StatusCode = StatusCode::UNPROCESSABLE_ENTITY;

#[tokio::test]
async fn subscribe_returns_a_bad_form_status_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=alexander%20gogas", "missing the email"),
        ("email=alexander.gogas%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            BAD_FORM_STATUS,
            response.status(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
