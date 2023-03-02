use std::env;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, NaiveDateTime};
use chrono_tz::Tz;
use crate::dates::Date;
use crate::games::Game;

static API_URL: &str = "https://statsapi.web.nhl.com/api/v1/schedule";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
  copyright: String,
  total_items: u32,
  total_events: u32,
  total_games: u32,
  total_matches: u32,
  meta_data : MetaData,
  wait: u32,
  dates: Vec<Date>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MetaData {
  time_stamp: String,
}

pub async fn get_todays_game_for_team(team_name: &String, date: &Option<String>) -> Option<String> {
    let schedule: Schedule = get_schedule(date).await;
    let date = schedule.dates.first();
    match date {
        None => None,
        Some(d) => {
            let game = get_team_game_from_schedule(d, team_name);
            match game {
                None => None,
                Some(g) => Some(generate_game_summary(g, team_name))
            }
        }
    }
}

async fn get_schedule(date: &Option<String>) -> Schedule {
    let mut endpoint = String::from(API_URL);
    if date.is_some() {
        let date_string = format!("?date={}", date.as_ref().unwrap());
        endpoint.push_str(date_string.as_str())
    }
    let resp = reqwest::get(endpoint)
    .await
    .expect("Failed to get schedule from NHL API")
    .text()
    .await
    .expect("Failed to convert schedule ressponse to text");

    return serde_json::from_str(&resp).unwrap();
}

fn get_team_game_from_schedule<'a>(date: &'a Date, team_name: &'a String) -> Option<&'a Game> {
    let games = &date.games;
    for game in games.iter() {
        if game.teams.home.team.name.eq(team_name) || game.teams.away.team.name.eq(team_name) {
            return Some(game)
        }
    }
    None
}

fn generate_game_summary(game: &Game, team_name: &String) -> String {
    let is_at_home = team_name.eq(&game.teams.home.team.name);

    let opposing_team: &String;
    if is_at_home {
        opposing_team = &game.teams.away.team.name;
    } else {
        opposing_team = &game.teams.home.team.name;
    }

    let venue = &game.venue.name;

    let date_time_utc = DateTime::<Utc>::from_utc(NaiveDateTime::parse_from_str(&game.game_date.as_str(), "%Y-%m-%dT%H:%M:%SZ").expect("Date could not be parsed"), Utc);
    let time_zone = Tz::from_str(env::var("TIME_ZONE").unwrap_or(String::from("UTC")).as_str()).expect("Invalid timezone");
    let localized_date_time = date_time_utc.with_timezone(&time_zone);
    let team_short_name = team_name.split_whitespace().next_back().unwrap_or(team_name);

    return format!("The {} play the {} today at {} at the {}.\nLet's go {}!!!", team_name, opposing_team, localized_date_time.format("%I:%M %p"), venue, team_short_name);
}