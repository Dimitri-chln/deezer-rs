use serde::Deserialize;

#[derive(Deserialize)]
pub struct List<T> {
    pub data: Vec<T>,
}
