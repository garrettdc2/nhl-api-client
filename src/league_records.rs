use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeagueRecord {
    wins: u32,
    losses: u32,
    ot: u32,
    #[serde(rename="type")]
    league_type: String
}