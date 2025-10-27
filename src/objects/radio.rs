use serde::Deserialize;

use crate::objects::traits;
use crate::{Id, Url};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Radio {
    // The radio deezer ID
    pub id: Id,
    // The radio title
    pub title: String,
    // The radio title
    pub description: String,
    // The share link of the radio on Deezer
    pub share: Url,
    // The url of the radio picture. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub picture: Url,
    // The url of the radio picture in size small.
    pub picture_small: Url,
    // The url of the radio picture in size medium.
    pub picture_medium: Url,
    // The url of the radio picture in size big.
    pub picture_big: Url,
    // The url of the radio picture in size xl.
    pub picture_xl: Url,
    // API Link to the tracklist of this radio	url
    pub tracklist: Url,
    ///
    pub md5_image: String,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::Object for Radio {
    const ENDPOINT: &str = "/radio";
}
