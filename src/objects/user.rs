use serde::Deserialize;

use crate::objects::{track::Country, traits};
use crate::{Id, Url};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct User {
    /// The user's Deezer ID
    pub id: Id,
    /// The user's Deezer nickname
    pub name: String,
    /// The user's last name
    pub lastname: String,
    /// The user's first name
    pub firstname: String,
    /// The user's email
    pub email: String,
    /// The user's status
    pub status: u32,
    /// The user's birthday
    pub birthday: String,
    /// The user's inscription date
    pub inscription_date: String,
    /// The user's gender
    pub gender: Gender,
    /// The url of the profil for the user on Deezer
    pub link: Url,
    /// The url of the user's profil picture. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub picture: Url,
    /// The url of the user's profil picture in size small.
    pub picture_small: Url,
    /// The url of the user's profil picture in size medium.
    pub picture_medium: Url,
    /// The url of the user's profil picture in size big.
    pub picture_big: Url,
    /// The url of the user's profil picture in size xl.
    pub picture_xl: Url,
    /// The user's country
    pub country: Country,
    /// The user's language
    pub lang: String,
    /// If the user is a kid or not
    pub is_kid: bool,
    /// The user's explicit content level according to his country
    pub explicit_content_level: ExplicitContentLevel,
    /// The user's available explicit content levels according to his country
    pub explicit_content_levels_available: Vec<ExplicitContentLevel>,
    /// API Link to the flow of this user
    pub tracklist: Url,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::Object for User {
    const ENDPOINT: &str = "/user";
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum Gender {
    F,
    M,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum ExplicitContentLevel {
    #[serde(alias = "explicit_display")]
    Display,
    #[serde(alias = "explicit_no_recommendation")]
    NoRecommendation,
    #[serde(alias = "explicit_hide")]
    Hide,
}
