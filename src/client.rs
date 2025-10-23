use reqwest::Url;

use crate::objects::Object;

#[derive(Debug, Clone)]
pub struct DeezerClient {
    client: reqwest::Client,
}

impl DeezerClient {
    /// Create a new unauthenticated client instance
    pub fn new() -> Self {
        DeezerClient {
            client: reqwest::Client::new(),
        }
    }

    pub(crate) async fn get<O: Object>(&self, url: Url) -> crate::Result<O> {
        let result = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<O>()
            .await?;

        Ok(result)
    }
}
