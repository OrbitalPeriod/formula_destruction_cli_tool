pub struct RaceResults{
    season : i32,
    race_id : i32,
    results : Vec<PersonRaceResult>,
}

pub struct PersonRaceResult{
    driver_id : i32,
    seat_id : Option<i32>,
    position : i32,
    bot_result : Option<bool>,
    pole : Option<bool>,
    leading_lap : Option<bool>,
    fastest_lap : Option<bool>,
    qualy_result : Option<i32>,
}   