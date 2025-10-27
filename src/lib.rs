mod client;
mod error;
mod image;
mod list;
pub mod objects;

pub use client::DeezerClient;
pub use error::DeezerError;
pub use list::*;

pub(crate) type Result<T> = std::result::Result<T, DeezerError>;
pub(crate) type Id = u32;
pub(crate) type Url = String;

const API_URL: &str = "https://api.deezer.com";
