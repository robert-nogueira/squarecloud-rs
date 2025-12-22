use std::sync::LazyLock;

pub struct Settings {
    pub base_url: String,
    pub api_token: String,
}

impl Settings {
    fn load() -> Self {
        Settings {
            api_token: dotenv::var("API_TOKEN").expect("Missing environment variable 'API_TOKEN'"),
            base_url: dotenv::var("BASE_URL").expect("Missing environment variable 'BASE_URL'"),
        }
    }
}

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(Settings::load);
