use integration_tests::enviornment::MockEnv;

#[tokio::test]
pub async fn health_check() {
    let mock_env = MockEnv::init().await;

    let url = format!("{}/health_check", mock_env.base_url);
    let response = reqwest::get(url).await.expect("reqwest::get");
    assert!(response.status().is_success())
}
