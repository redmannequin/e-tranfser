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
                    "payer": {
                        "full_name": "Bob Burge",
                        "email": "bob.burge@email.com"
                    },
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

    assert!(response.status().is_success());
    let payment: serde_json::Value = response.json().await.expect("parse response");

    let url = format!("{}/deposit_payment", mock_env.base_url);
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(format!(
            r#"
                {{
                    "payment_id": "{}",
                    "security_answer": "superman",
                    "iban": "{}"

                }}
            "#,
            payment["payment_id"].as_str().expect("blah"),
            "fake_iban"
        ))
        .send()
        .await
        .expect("reqwest::post");

    assert!(response.status().is_success());

    let url = format!("{}/deposit_payment", mock_env.base_url);
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(format!(
            r#"
                {{
                    "payment_id": "{}",
                    "security_answer": "superman",
                    "iban": "{}"

                }}
            "#,
            payment["payment_id"].as_str().expect("blah"),
            "fake_iban"
        ))
        .send()
        .await
        .expect("reqwest::post");

    assert!(response.status().is_client_error())
}
