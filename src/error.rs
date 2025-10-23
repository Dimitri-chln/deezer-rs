use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeezerError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}
