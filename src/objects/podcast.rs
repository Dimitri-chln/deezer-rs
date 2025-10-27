use serde::Deserialize;

use crate::objects::traits;
use crate::{Id, Url};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Podcast {
    /// The podcast's Deezer id
    pub id: Id,
    /// The podcast's title
    pub title: String,
    /// The podcast's description
    pub description: String,
    /// If the podcast is available or not
    pub available: bool,
    /// The number of podcast's fans
    pub fans: u32,
    /// The url of the podcast on Deezer
    pub link: Url,
    /// The share link of the podcast on Deezer
    pub share: Url,
    /// The url of the podcast's cover.
    pub picture: Url,
    /// The url of the podcast's cover in size small.
    pub picture_small: Url,
    /// The url of the podcast's cover in size medium.
    pub picture_medium: Url,
    /// The url of the podcast's cover in size big.
    pub picture_big: Url,
    /// The url of the podcast's cover in size xl.
    pub picture_xl: Url,
}

impl traits::Object for Podcast {
    const ENDPOINT: &str = "/podcast";
}
