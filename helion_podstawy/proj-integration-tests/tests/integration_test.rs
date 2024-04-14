use proj_integration_tests::api_client::ApiClient;

#[test]
fn test_api_client_get_user_data() {
    let client = ApiClient::new("http://example.com/api");
    let user_data = client.get_user_data(1);

    assert!(user_data.is_ok());
    let data = user_data.unwrap();

    assert!(data.contains("Example Domain"));
}