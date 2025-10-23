use reqwest::Url;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::objects::{Artist, Genre, IncompleteObject, Object};
use crate::{Id, List};

/// type: "album"
#[derive(Deserialize)]
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
    /// The url of the album's cover. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub cover: Url,
    /// The url of the album's cover in size small.
    pub cover_small: Url,
    /// The url of the album's cover in size medium.
    pub cover_medium: Url,
    /// The url of the album's cover in size big.
    pub cover_big: Url,
    /// The url of the album's cover in size xl.
    pub cover_xl: Url,
    ///
    pub md5_image: String,
    /// The album's first genre id (You should use the genre list instead). NB : -1 for not found
    pub genre_id: i32,
    /// List of genre object
    pub genres: List<AlbumGenre>,
    /// The album's label name
    pub label: String,
    /// The album's provider name
    pub provider: String,
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
    pub alternative: Option<Box<Album>>,
    /// API Link to the tracklist of this album
    pub tracklist: Url,
    /// Whether the album contains explicit lyrics
    pub explicit_lyrics: bool,
    /// The explicit content lyrics values
    pub explicit_content_lyrics: AlbumExplicitContent,
    /// The explicit cover values
    pub explicit_content_cover: AlbumExplicitContent,
    /// Return a list of contributors on the album
    pub contributors: Vec<AlbumContributor>,
    /// Return fallback album with id and status
    pub fallback: Option<AlbumFallback>,
    /// [`Artist`](crate::objects::Artist) object containing : id, name, picture, picture_small, picture_medium, picture_big, picture_xl
    pub artist: AlbumArtist,
    /// list of track
    pub tracks: List<AlbumTrack>,
}

impl Object for Album {
    const ENDPOINT: &str = "/album";
}

#[derive(Deserialize_repr)]
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

// type: "genre"
#[derive(Deserialize)]
pub struct AlbumGenre {
    /// The genre's Deezer id
    pub id: Id,
    /// The genre's name
    pub name: String,
    /// The url of the genre picture. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub picture: Url,
}

impl IncompleteObject<Genre> for AlbumGenre {
    fn id(&self) -> Id {
        self.id
    }
}

// type: "artist"
#[derive(Deserialize)]
pub struct AlbumContributor {
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
    /// true if the artist has a smartradio
    pub radio: bool,
    /// API Link to the top of this artist
    pub tracklist: Url,

    /// ?
    pub role: String,
}

impl IncompleteObject<Artist> for AlbumContributor {
    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Deserialize)]
pub struct AlbumFallback {
    // TODO
}

// type: "artist"
#[derive(Deserialize)]
pub struct AlbumArtist {
    /// The artist's Deezer id
    pub id: Id,
    /// The artist's name
    pub name: String,
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
    /// API Link to the top of this artist
    pub tracklist: Url,
}

impl IncompleteObject<Artist> for AlbumArtist {
    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Deserialize)]
pub struct AlbumTrack {
    /// The track's Deezer id
    pub id: Id,
    // TODO
}
