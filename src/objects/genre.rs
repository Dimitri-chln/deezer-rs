use serde::Deserialize;

use crate::objects::Object;

// type: "genre"
#[derive(Deserialize)]
pub struct Genre {}

impl Object for Genre {
    const ENDPOINT: &str = "/genre";
}
