use crate::Url;
use serde::Deserialize;

use crate::objects::traits;

// type: "episode"
/// An episode object
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Episode {
    /// The episode's Deezer id
    pub id: u32,
    /// The episode's title
    pub title: String,
    /// The episode's description
    pub description: String,
    /// If the episode is available or not
    pub available: bool,
    /// The episode's release date
    pub release_date: String,
    /// The episode's duration (seconds)
    pub duration: u32,
    /// The url of the episode on Deezer
    pub link: Url,
    /// The share link of the episode on Deezer
    pub share: Url,
    /// The url of the episode's cover.
    pub picture: Url,
    /// The url of the episode's cover in size small.
    pub picture_small: Url,
    /// The url of the episode's cover in size medium.
    pub picture_medium: Url,
    /// The url of the episode's cover in size big.
    pub picture_big: Url,
    /// The url of the episode's cover in size xl.
    pub picture_xl: Url,
    /// The track token for media service
    pub track_token: String,
    /// [Podcast](crate::objects::Podcast) object containing : id, title, link, picture, picture_small, picture_medium, picture_big, picture_xl
    pub podcast: EpisodePodcast,
}

impl traits::Object for Episode {
    const ENDPOINT: &str = "/episode";
}

// type: "podcast"
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EpisodePodcast {
    // TODO
}
