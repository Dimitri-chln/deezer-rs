mod client;
mod error;
mod list;
pub mod objects;

pub use self::client::DeezerClient;
pub use self::error::DeezerError;

pub use list::List;

pub(crate) type Result<T> = std::result::Result<T, DeezerError>;
pub(crate) type Id = u32;

const API_URL: &str = "https://api.deezer.com";
