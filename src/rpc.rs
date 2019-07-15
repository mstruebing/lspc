pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

mod client;
pub use client::Message;
pub use client::Client;