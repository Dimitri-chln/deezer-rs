use std::num::NonZeroU32;

use serde::Deserialize;

/// Get the user's options
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Options {
    /// If the user can stream on the platform
    pub streaming: bool,
    /// the streaming duration of the user
    pub streaming_duration: u32,
    /// The user can listen to the music in offline mode
    pub offline: bool,
    /// The HQ can be activated
    pub hq: bool,
    /// Displays ads
    pub ads_display: bool,
    /// Activates audio ads
    pub ads_audio: bool,
    /// If the user reached the limit of linked devices
    pub too_many_devices: bool,
    /// If the user can subscribe to the service
    pub can_subscribe: bool,
    /// The limit of radio skips.
    pub radio_skips: Option<NonZeroU32>,
    /// Lossless is available
    pub lossless: bool,
    /// Allows to display the preview of the tracks
    pub preview: bool,
    /// Allows to stream the radio
    pub radio: bool,
}
