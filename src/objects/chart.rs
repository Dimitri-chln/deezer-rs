use serde::Deserialize;

use crate::objects::traits;
use crate::objects::{Album, Artist, Track, track::TrackExplicitContent};
use crate::{Id, ListWithTotal, Url};

/// Charts of a specified genre
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Chart {
    /// list of track
    pub tracks: ListWithTotal<ChartTrack>,
    /// list of album
    pub albums: ListWithTotal<ChartAlbum>,
    /// list of artist
    pub artists: ListWithTotal<ChartArtist>,
    /// list of playlist
    pub playlists: ListWithTotal<ChartPlaylist>,
    /// list of podcast
    pub podcasts: ListWithTotal<ChartPodcast>,
}

impl traits::Object for Chart {
    const ENDPOINT: &str = "/chart";
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChartTrack {
    /// The track's Deezer id
    pub id: Id,
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
    /// The position of the track in the charts
    pub position: u32,
    /// [`Artist`](crate::objects::Artist) object containing : id, name, link, picture, picture_small, picture_medium, picture_big, picture_xl, radio
    pub artist: ChartTrackArtist,
    /// [`Album`](crate::objects::Album) object containing : id, title, cover, cover_small, cover_medium, cover_big, cover_xl
    pub album: ChartTrackAlbum,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::IncompleteObject for ChartTrack {
    type FullObject = Track;

    fn id(&self) -> Id {
        self.id
    }
}

/// [`Artist`](crate::objects::Artist) object containing : id, name, link, picture, picture_small, picture_medium, picture_big, picture_xl, radio
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChartTrackArtist {
    /// The artist's Deezer id
    pub id: Id,
    /// The artist's name
    pub name: String,
    /// The url of the artist on Deezer
    pub link: Url,
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

    #[allow(dead_code)]
    r#type: String,
}

impl traits::IncompleteObject for ChartTrackArtist {
    type FullObject = Artist;

    fn id(&self) -> Id {
        self.id
    }
}

/// [Album](crate::objects::Album) object containing : id, title, cover, cover_small, cover_medium, cover_big, cover_xl
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChartTrackAlbum {
    /// The Deezer album id
    pub id: Id,
    /// The album title
    pub title: String,
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
    ///
    pub tracklist: Url,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::IncompleteObject for ChartTrackAlbum {
    type FullObject = Album;

    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChartAlbum {
    /// The Deezer album id
    pub id: Id,
    /// The album title
    pub title: String,
    /// The url of the album on Deezer
    pub link: Url,
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
    /// The record type of the album (EP / ALBUM / etc..)
    pub record_type: String,
    /// API Link to the tracklist of this album
    pub tracklist: Url,
    /// Whether the album contains explicit lyrics
    pub explicit_lyrics: bool,
    /// The position of the album in the charts
    pub position: u32,
    /// [`Artist`](crate::objects::Artist) object containing : id, name, link, picture, picture_small, picture_medium, picture_big, picture_xl, radio
    pub artist: ChartAlbumArtist,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::IncompleteObject for ChartAlbum {
    type FullObject = Album;

    fn id(&self) -> Id {
        self.id
    }
}

/// [`Artist`](crate::objects::Artist) object containing : id, name, link, picture, picture_small, picture_medium, picture_big, picture_xl, radio
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChartAlbumArtist {
    /// The artist's Deezer id
    pub id: Id,
    /// The artist's name
    pub name: String,
    /// The url of the artist on Deezer
    pub link: Url,
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

    #[allow(dead_code)]
    r#type: String,
}

impl traits::IncompleteObject for ChartAlbumArtist {
    type FullObject = Artist;

    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChartArtist {
    /// The artist's Deezer id
    pub id: Id,
    /// The artist's name
    pub name: String,
    /// The url of the artist on Deezer
    pub link: Url,
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
    /// The position of the artist in the charts
    pub position: u32,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::IncompleteObject for ChartArtist {
    type FullObject = Artist;

    fn id(&self) -> Id {
        self.id
    }
}

// type: "playlist"
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChartPlaylist {
    // TODO
}

// type: "podcast"
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChartPodcast {
    // TODO
}
