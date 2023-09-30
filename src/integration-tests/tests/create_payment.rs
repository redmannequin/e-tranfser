use integration_tests::enviornment::MockEnv;

#[tokio::test]
async fn create_payment() {
    let mock_env = MockEnv::init().await;

    let url = format!("{}/create_payment", mock_env.base_url);
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(
            r#"
                {
                    "full_name": "John Doe",
                    "email": "john.doe@email.com",
                    "amount": 1000,
                    "security_question": "Whats your dogs name",
                    "security_answer": "superman"
                }
            "#,
        )
        .send()
        .await
        .expect("reqwest::post");

    dbg!("{}", &response);
    assert!(response.status().is_success())
}
