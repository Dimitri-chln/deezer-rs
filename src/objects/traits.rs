use serde::de::DeserializeOwned;

use crate::{API_URL, DeezerClient, Id, Url};

pub trait Object: DeserializeOwned {
    const ENDPOINT: &str;

    fn endpoint(id: Id) -> Url {
        format!("{API_URL}{endpoint}/{id}", endpoint = Self::ENDPOINT)
    }

    fn get(id: Id, client: &DeezerClient) -> impl Future<Output = crate::Result<Self>> + Send {
        client.get::<Self>(id)
    }
}

pub trait IncompleteObject {
    type FullObject: Object;

    fn id(&self) -> Id;

    fn full(
        &self,
        client: &DeezerClient,
    ) -> impl Future<Output = crate::Result<Self::FullObject>> + Send {
        client.get::<Self::FullObject>(self.id())
    }
}
