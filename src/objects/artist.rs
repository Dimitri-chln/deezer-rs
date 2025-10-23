use reqwest::Url;
use serde::Deserialize;

use crate::{Id, objects::Object};

// type: "artist"
#[derive(Deserialize)]
pub struct Artist {
    /// The artist's Deezer id
    pub id: Id,
    /// The artist's name
    pub name: String,
    /// The url of the artist on Deezer
    pub link: Url,
    /// The share link of the artist on Deezer
    pub share: Url,
    /// The url of the artist picture. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub picture: Url,
    /// The url of the artist picture in size small.
    pub picture_small: Url,
    /// The url of the artist picture in size medium.
    pub picture_medium: Url,
    /// The url of the artist picture in size big.
    pub picture_big: Url,
    /// The url of the artist picture in size xl.
    pub picture_xl: Url,
    /// The number of artist's albums
    pub nb_albums: u32,
    /// The number of artist's fans
    pub nb_fans: u32,
    /// true if the artist has a smartradio
    pub radio: bool,
    /// API Link to the top of this artist
    pub tracklist: Url,
}

impl Object for Artist {
    const ENDPOINT: &str = "/artist";
}
