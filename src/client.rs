use crate::{Id, Url};

use crate::objects::traits;

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

    async fn get_from_url<O>(&self, url: Url) -> crate::Result<O>
    where
        O: traits::Object,
    {
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

    pub async fn get<O>(&self, id: Id) -> crate::Result<O>
    where
        O: traits::Object,
    {
        let endpoint = O::endpoint(id);
        let object = self.get_from_url(endpoint).await?;

        Ok(object)
    }
}
