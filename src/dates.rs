use serde::{Serialize, Deserialize};
use crate::games::Game;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    date: String,
    total_items: u32,
    total_events: u32,
    total_games: u32,
    total_matches: u32,
    pub games: Vec<Game>,
}