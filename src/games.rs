use serde::{Serialize, Deserialize};
use crate::league_records::LeagueRecord;
use crate::venues::Venue;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    game_pk: u32,
    link: String,
    game_type: String,
    season: String,
    pub game_date: String,
    status: Status,
    pub teams: Teams,
    pub venue: Venue,
    content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    abstract_game_state: String,
    coded_game_state: String,
    detailed_state: String,
    status_code: String,
    #[serde(rename="startTimeTBD")]
    start_time_tbd: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Teams {
    pub home: TeamStats,
    pub away: TeamStats,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    id: u32,
    pub name: String,
    link: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamStats {
    league_record: LeagueRecord,
    score: u32,
    pub team: Team,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    link: String,
}