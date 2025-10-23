mod album;
mod artist;
mod genre;

pub use album::Album;
pub use artist::Artist;
pub use genre::Genre;

use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::{API_URL, DeezerClient, Id};

pub trait Object: DeserializeOwned {
    const ENDPOINT: &str;

    fn endpoint(id: Id) -> Url {
        let endpoint = format!("{API_URL}{endpoint}/{id}", endpoint = Self::ENDPOINT);
        Url::parse(&endpoint).expect("URL should be well formed")
    }
}

pub trait IncompleteObject<O: Object> {
    fn id(&self) -> Id;

    fn full(&self, client: &DeezerClient) -> impl Future<Output = crate::Result<O>> + Send
    where
        Self: Sync,
    {
        async {
            let endpoint = O::endpoint(self.id());
            client.get(endpoint).await
        }
    }
}
