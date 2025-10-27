use serde::Deserialize;

use crate::objects::traits;
use crate::objects::{Album, Artist, Track, User, track::TrackExplicitContent};
use crate::{Id, ListWithChecksum, Url};

/// A playlist object
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Playlist {
    /// The playlist's Deezer id
    pub id: Id,
    /// The playlist's title
    pub title: String,
    /// The playlist description
    pub description: String,
    /// The playlist's duration (seconds)
    pub duration: u32,
    /// If the playlist is public or not
    pub public: bool,
    /// If the playlist is the love tracks playlist
    pub is_loved_track: bool,
    /// If the playlist is collaborative or not
    pub collaborative: bool,
    /// Nb tracks in the playlist
    pub nb_tracks: u32,
    /// Nb tracks not seen
    pub unseen_track_count: Option<u32>,
    /// The number of playlist's fans
    pub fans: u32,
    /// The url of the playlist on Deezer
    pub link: Url,
    /// The share link of the playlist on Deezer
    pub share: Url,
    /// The url of the playlist's cover. Add 'size' parameter to the url to change size. Can be 'small', 'medium', 'big', 'xl'
    pub picture: Url,
    /// The url of the playlist's cover in size small.
    pub picture_small: Url,
    /// The url of the playlist's cover in size medium.
    pub picture_medium: Url,
    /// The url of the playlist's cover in size big.
    pub picture_big: Url,
    /// The url of the playlist's cover in size xl.
    pub picture_xl: Url,
    /// The checksum for the track list
    pub checksum: String,
    ///
    pub tracklist: Url,
    ///
    pub creation_date: String,
    ///
    pub add_date: String,
    ///
    pub mod_date: String,
    ///
    pub md5_image: String,
    ///
    pub picture_type: String,
    /// [User](crate::objects::User) object containing : id, name
    pub creator: PlaylistCreator,
    /// list of track
    pub tracks: ListWithChecksum<PlaylistTrack>,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::Object for Playlist {
    const ENDPOINT: &str = "/playlist";
}

/// [User](crate::objects::User) object containing : id, name
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaylistCreator {
    /// The artist's Deezer id
    pub id: Id,
    /// The artist's name
    pub name: String,
    /// API Link to the top of this artist
    pub tracklist: Url,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::IncompleteObject for PlaylistCreator {
    type FullObject = User;

    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaylistTrack {
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
    /// The track unseen status
    pub unseen: Option<bool>,
    /// The track isrc
    pub isrc: String,
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
    /// The time when the track has been added to the playlist
    pub time_add: u64,
    /// [`Artist`](crate::objects::Artist) object containing : id, name, link
    pub artist: PlaylistTrackArtist,
    /// [`Album`](crate::objects::Album) object containing : id, title, upc, cover, cover_small, cover_medium, cover_big, cover_xl
    pub album: PlaylistTrackAlbum,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::IncompleteObject for PlaylistTrack {
    type FullObject = Track;

    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaylistTrackArtist {
    /// The artist's Deezer id
    pub id: Id,
    /// The artist's name
    pub name: String,
    /// The url of the artist on Deezer
    pub link: Url,
    /// API Link to the top of this artist
    pub tracklist: Url,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::IncompleteObject for PlaylistTrackArtist {
    type FullObject = Artist;

    fn id(&self) -> Id {
        self.id
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaylistTrackAlbum {
    /// The Deezer album id
    pub id: Id,
    /// The album title
    pub title: String,
    /// The album UPC
    pub upc: String,
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
    /// API Link to the tracklist of this album
    pub tracklist: Url,

    #[allow(dead_code)]
    r#type: String,
}

impl traits::IncompleteObject for PlaylistTrackAlbum {
    type FullObject = Album;

    fn id(&self) -> Id {
        self.id
    }
}
