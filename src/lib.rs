mod schedules;
mod dates;
mod games;
mod league_records;
mod venues;

pub struct Client {
    default_team: String
}

impl Client {
    pub fn new(default_team: String) -> Client {
        Client { default_team: default_team }
    }

    pub async fn get_todays_game(&self) -> Option<String> {
        schedules::get_todays_game_for_team(&self.default_team, &None).await
    }
}