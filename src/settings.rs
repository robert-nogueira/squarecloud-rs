use std::sync::LazyLock;

pub struct Settings {
    pub api_token: String,
}

impl Settings {
    fn load() -> Self {
        Settings {
            api_token: dotenvy::var("API_TOKEN")
                .expect("Missing environment variable 'API_TOKEN'"),
        }
    }
}

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(Settings::load);
