use deezer_rs_macros::{DeezerIncomplete, DeezerObject};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::{
    Id, List, Url,
    image::{Cover, Picture},
    objects::{Artist, Genre, Track, artist::Contributor, track::TrackExplicitContent},
};

/// An album object
#[derive(DeezerObject, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[deezer(endpoint = "/album")]
pub struct Album {
    /// The Deezer album id
    pub id: Id,

    /// The album title
    pub title: String,

    /// The album UPC
    pub upc: String,

    /// The url of the album on Deezer
    pub link: Url,

    /// The share link of the album on Deezer
    pub share: Url,

    #[serde(flatten)]
    pub cover: Cover,

    ///
    pub md5_image: String,

    /// The album's first genre id (You should use the genre list instead). NB : -1 for not found
    pub genre_id: i32,

    /// List of genre object
    pub genres: List<AlbumGenre>,

    /// The album's label name
    pub label: String,

    /// The album's provider name
    pub provider: Option<String>,

    ///
    pub nb_tracks: u32,

    /// The album's duration (seconds)
    pub duration: u32,

    /// The number of album's Fans
    pub fans: u32,

    /// The album's release date
    pub release_date: String,

    /// The record type of the album (EP / ALBUM / etc..)
    pub record_type: String,

    ///
    pub available: bool,

    /// Return an alternative album object if the current album is not available
    pub alternative: Option<Box<Self>>,

    /// API Link to the tracklist of this album
    pub tracklist: Url,

    /// Whether the album contains explicit lyrics
    pub explicit_lyrics: bool,

    /// The explicit content lyrics values
    pub explicit_content_lyrics: AlbumExplicitContent,

    /// The explicit cover values
    pub explicit_content_cover: AlbumExplicitContent,

    /// Return a list of contributors on the album
    pub contributors: Vec<Contributor>,

    /// Return fallback album with id and status
    pub fallback: Option<AlbumFallback>,

    /// [`Artist`](crate::objects::Artist) object containing : id, name, picture, picture_small, picture_medium, picture_big, picture_xl
    pub artist: AlbumArtist,

    /// list of track
    pub tracks: List<AlbumTrack>,

    #[allow(dead_code)]
    r#type: String,
}

#[derive(Deserialize_repr, Debug)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum AlbumExplicitContent {
    NotExplicit = 0,
    Explicit = 1,
    Unknown = 2,
    Edited = 3,
    NoAdviceAvailable = 6,

    /// Album "lyrics" only
    PartiallyExplicit = 4,
    /// Album "lyrics" only
    PartiallyUnknown = 5,
    /// Album "lyrics" only
    PartiallyNoAdviceAvailable = 7,
}

#[derive(DeezerIncomplete, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[deezer(object = Genre)]
pub struct AlbumGenre {
    /// The genre's Deezer id
    pub id: Id,

    /// The genre's name
    pub name: String,

    /// The url of the genre picture. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub picture: Url,

    #[allow(dead_code)]
    r#type: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AlbumFallback {
    // TODO
}

/// [`Artist`](crate::objects::Artist) object containing : id, name, picture, picture_small, picture_medium, picture_big, picture_xl
#[derive(DeezerIncomplete, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[deezer(object = Artist)]
pub struct AlbumArtist {
    /// The artist's Deezer id
    pub id: Id,

    /// The artist's name
    pub name: String,

    #[serde(flatten)]
    pub picture: Picture,

    /// API Link to the top of this artist
    pub tracklist: Url,

    #[allow(dead_code)]
    r#type: String,
}

#[derive(DeezerIncomplete, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[deezer(object = Track)]
pub struct AlbumTrack {
    /// The track's Deezer id
    pub id: Id,

    /// true if the track is readable in the player for the current user
    pub readable: bool,

    /// The track's full title
    pub title: String,

    /// The track's short title
    pub title_short: String,

    /// The track version
    pub title_version: String,

    /// The url of the track on Deezer
    pub link: Url,

    /// The track's duration in seconds
    pub duration: u32,

    /// The track's Deezer rank
    pub rank: u32,

    /// Whether the track contains explicit lyrics
    pub explicit_lyrics: bool,

    /// The explicit content lyrics values
    pub explicit_content_lyrics: TrackExplicitContent,

    /// The explicit cover value
    pub explicit_content_cover: TrackExplicitContent,

    /// The url of track's preview file. This file contains the first 30 seconds of the track
    pub preview: Url,

    ///
    pub md5_image: String,

    /// [Artist](crate::objects::Artist) object containing : id, name
    pub artist: AlbumTrackArtist,

    /// [Album](crate::objects::Album) object containing : id, title, cover, cover_small, cover_medium, cover_big, cover_xl
    pub album: AlbumTrackAlbum,

    #[allow(dead_code)]
    r#type: String,
}

/// [Artist](crate::objects::Artist) object containing : id, name
#[derive(DeezerIncomplete, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[deezer(object = Artist)]
pub struct AlbumTrackArtist {
    /// The artist's Deezer id
    pub id: Id,

    /// The artist's name
    pub name: String,

    /// API Link to the top of this artist
    pub tracklist: Url,

    #[allow(dead_code)]
    r#type: String,
}

/// [Album](crate::objects::Album) object containing : id, title, cover, cover_small, cover_medium, cover_big, cover_xl
#[derive(DeezerIncomplete, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[deezer(object = Album)]
pub struct AlbumTrackAlbum {
    /// The Deezer album id
    pub id: Id,

    /// The album title
    pub title: String,

    #[serde(flatten)]
    pub cover: Cover,

    ///
    pub md5_image: String,

    ///
    pub tracklist: Url,

    #[allow(dead_code)]
    r#type: String,
}
