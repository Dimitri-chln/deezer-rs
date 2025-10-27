use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct List<T> {
    pub data: Vec<T>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListWithTotal<T> {
    pub data: Vec<T>,
    pub total: u32,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListWithChecksum<T> {
    pub data: Vec<T>,
    pub checksum: String,
}
