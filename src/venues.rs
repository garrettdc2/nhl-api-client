use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    #[serde(default)]
    id: Option<u32>,
    pub name: String,
    link: String,
}