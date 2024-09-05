use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RaceResults {
    pub season: i32,
    pub race_id: i32,
    pub results: Vec<PersonRaceResult>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct PersonRaceResult {
    pub driver_id: i32,
    pub seat_id: Option<i32>,
    pub position: i32,
    pub bot_result: Option<bool>,
    pub pole: Option<bool>,
    pub leading_lap: Option<bool>,
    pub fastest_lap: Option<bool>,
    pub qualy_result: Option<i32>,
}

impl From<PersonRaceResult> for RaceResult {
    fn from(value: PersonRaceResult) -> Self {
        RaceResult {
            position: value.position,
            bot_result: value.bot_result.unwrap_or(false),
            pole: value.pole.unwrap_or(false),
            leading_lap: value.leading_lap.unwrap_or(false),
            fastest_lap: value.fastest_lap.unwrap_or(false),
            qualy_result: value.qualy_result.unwrap_or(20),
        }
    }
}

pub struct RaceResult {
    pub position: i32,
    pub bot_result: bool,
    pub pole: bool,
    pub leading_lap: bool,
    pub fastest_lap: bool,
    pub qualy_result: i32,
}

pub fn read_race_result(filepath: &str) -> Result<RaceResults, Box<dyn Error>> {
    let file_string = std::fs::read_to_string(filepath).map_err(Box::new)?;
    let result: RaceResults = serde_json::from_str(&file_string).map_err(Box::new)?;

    Ok(result)
}
