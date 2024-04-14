pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        ApiClient {
            base_url: base_url.to_string(),
        }
    }

    pub fn get_user_data(&self, user_id: i32) -> Result<String, reqwest::Error> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        reqwest::blocking::get(url)?.text()
    }
}