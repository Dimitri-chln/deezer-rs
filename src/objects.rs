mod album;
mod artist;
mod chart;
mod editorial;
mod episode;
mod genre;
mod infos;
mod options;
mod playlist;
mod podcast;
mod radio;
mod track;
mod user;

pub mod traits;

pub use album::Album;
pub use artist::Artist;
pub use chart::Chart;
pub use editorial::Editorial;
pub use episode::Episode;
pub use genre::Genre;
pub use infos::Infos;
pub use options::Options;
pub use playlist::Playlist;
pub use podcast::Podcast;
pub use radio::Radio;
pub use track::Track;
pub use user::User;

use serde::Deserialize;

#[cfg(debug_assertions)]
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Unknown;
