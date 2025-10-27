use crate::Url;
use serde::Deserialize;

use crate::objects::traits;

// type: "editorial"
/// An editorial object
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Editorial {
    /// The editorial's Deezer id
    pub id: u32,
    /// The editorial's name
    pub name: String,
    /// The url of the editorial picture.
    pub picture: Url,
    /// The url of the editorial picture in size small.
    pub picture_small: Url,
    /// The url of the editorial picture in size medium.
    pub picture_medium: Url,
    /// The url of the editorial picture in size big.
    pub picture_big: Url,
    /// The url of the editorial picture in size xl.
    pub picture_xl: Url,
}

impl traits::Object for Editorial {
    const ENDPOINT: &str = "/editorial";
}
