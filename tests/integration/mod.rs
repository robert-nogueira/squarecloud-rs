use std::sync::Once;

mod account;

static ENV: Once = Once::new();

pub fn setup() {
    ENV.call_once(|| {
        dotenvy::from_filename(".env.test")
            .expect(".env.test not found — copy .env.test.example");
    });
}
