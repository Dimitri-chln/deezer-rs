use serde::Deserialize;

use crate::objects::Unknown;

/// Get the information about the API in the current country
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Infos {
    /// The current country ISO code
    pub country_iso: String,
    /// The current country name
    pub country: String,
    /// Indicates if Deezer is open in the current country or not
    pub open: bool,
    /// An array of available offers in the current country
    pub offers: Vec<Unknown>,
}
