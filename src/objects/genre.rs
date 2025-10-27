use crate::Url;
use serde::Deserialize;

use crate::objects::traits;

// type: "genre"
/// A genre object
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Genre {
    /// The genre's Deezer id
    pub id: u32,
    /// The genre's name
    pub name: String,
    /// The url of the genre picture. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub picture: Url,
    /// The url of the genre picture in size small.
    pub picture_small: Url,
    /// The url of the genre picture in size medium.
    pub picture_medium: Url,
    /// The url of the genre picture in size big.
    pub picture_big: Url,
    /// The url of the genre picture in size xl.
    pub picture_xl: Url,
}

impl traits::Object for Genre {
    const ENDPOINT: &str = "/genre";
}
