mod http;
pub mod resources;
mod settings;
pub mod types;

pub use http::endpoints::Endpoint;

#[cfg(test)]
mod tests {
    // use super::*;
}
