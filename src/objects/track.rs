use crate::Url;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::Id;
use crate::objects::traits;
use crate::objects::{Album, Artist, artist::Contributor};

// type: "track"
/// A track object
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Track {
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
    pub unseen: bool,
    /// The track isrc
    pub isrc: String,
    /// The url of the track on Deezer
    pub link: Url,
    /// The share link of the track on Deezer
    pub share: Url,
    /// The track's duration in seconds
    pub duration: u32,
    /// The track's album's disk number
    pub disk_number: u32,
    /// The track's Deezer rank
    pub rank: u32,
    /// The track's release date
    pub release_date: String,
    /// Whether the track contains explicit lyrics
    pub explicit_lyrics: bool,
    /// The explicit content lyrics values
    pub explicit_content_lyrics: TrackExplicitContent,
    /// The explicit cover value
    pub explicit_content_cover: TrackExplicitContent,
    /// The url of track's preview file. This file contains the first 30 seconds of the track
    pub preview: Url,
    /// Beats per minute
    pub bpm: f32,
    /// Signal strength
    pub gain: f32,
    /// List of countries where the track is available
    pub available_countries: Vec<Country>,
    /// Return an alternative readable track if the current track is not readable
    pub alternative: Option<Box<Self>>,
    /// Return a list of contributors on the track
    pub contributors: Vec<Contributor>,
    ///
    pub md5_image: String,
    /// The track token for media service
    pub track_token: String,
    /// [`Artist`](crate::objects::Artist) object containing : id, name, link, share, picture, picture_small, picture_medium, picture_big, picture_xl, nb_album, nb_fan, radio, tracklist, role
    pub artist: TrackArtist,
    /// [`Album`](crate::objects::Album) object containing : id, title, link, cover, cover_small, cover_medium, cover_big, cover_xl, release_date
    pub album: TrackAlbum,
}

impl traits::Object for Track {
    const ENDPOINT: &str = "/track";
}

#[derive(Deserialize_repr, Debug)]
#[serde(deny_unknown_fields)]
#[repr(u8)]
pub enum TrackExplicitContent {
    NotExplicit = 0,
    Explicit = 1,
    Unknown = 2,
    Edited = 3,
    NoAdviceAvailable = 6,
}

#[rustfmt::skip]
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum Country {
    AE, AF, AG, AI, AL, AM, AO, AQ, AR, AS, AT, AU, AZ,
    BA, BB, BD, BE, BF, BG, BH, BI, BJ, BN, BO, BQ, BR, BT, BV, BW, BY,
    CA, CC, CD, CF, CG, CH, CI, CK, CL, CM, CN, CO, CR, CV, CW, CX, CY, CZ,
    DE, DJ, DK, DM, DO, DZ,
    EC, EE, EG, EH, ER, ES, ET,
    FI, FJ, FK, FM, FR,
    GA, GB, GD, GE, GH, GM, GN, GQ, GR, GS, GT, GU, GW,
    HK, HM, HN, HR, HU,
    ID, IE, IL, IN, IO, IQ, IS, IT,
    JM, JO, JP,
    KE, KG, KH, KI, KM, KN, KR, KW, KY, KZ,
    LA, LB, LC, LK, LR, LS, LT, LU, LV, LY,
    MA, MD, ME, MG, MH, MK, ML, MM, MN, MP, MR, MS, MT, MU, MV, MW, MX, MY, MZ,
    NA, NE, NF, NG, NI, NL, NO, NP, NR, NU, NZ,
    OM,
	PA, PE, PG, PH, PK, PL, PN, PS, PT, PW, PY,
    QA,
    RO, RS, RU, RW,
    SA, SB, SC, SD, SE, SG, SI, SJ, SK, SL, SN, SO, SS, ST, SV, SX, SZ,
    TC, TD, TG, TH, TJ, TK, TL, TM, TN, TO, TR, TV, TW, TZ,
    UA, UG, US, UY, UZ,
    VC, VE, VG, VI, VN, VU,
    WS,
    YE,
    ZA, ZM, ZW,
}

// type: "artist"
/// [`Artist`](crate::objects::Artist) object containing : id, name, link, share, picture, picture_small, picture_medium, picture_big, picture_xl, nb_album, nb_fan, radio, tracklist, role
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrackArtist {
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
    pub nb_album: u32,
    /// The number of artist's fans
    pub nb_fan: u32,
    /// true if the artist has a smartradio
    pub radio: bool,
    /// API Link to the top of this artist
    pub tracklist: Url,

    /// ?
    pub role: String,
}

impl traits::IncompleteObject for TrackArtist {
    type FullObject = Artist;

    fn id(&self) -> Id {
        self.id
    }
}

// type: "album"
/// [`Album`](crate::objects::Album) object containing : id, title, link, cover, cover_small, cover_medium, cover_big, cover_xl, release_date
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrackAlbum {
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
    /// The album's release date
    pub release_date: String,
}

impl traits::IncompleteObject for TrackAlbum {
    type FullObject = Album;

    fn id(&self) -> Id {
        self.id
    }
}
