use std::error::Error;

use fileparser::{PersonRaceResult, RaceResult};
use sqlx::{Executor, Pool, Postgres, Transaction};
mod fileparser;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub mode: ConfigMode,
    pub filepath: String,
}
#[derive(Debug)]
pub enum ConfigMode {
    RaceResult,
}

pub async fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let pool = get_db_connection(config).await.map_err(Box::new)?;

    match config.mode {
        ConfigMode::RaceResult => insert_race_results(pool, config).await,
    }
}

#[derive(Debug, derive_more::Display)]
pub enum LibError {
    TransactionError,
    RaceDoesntExist,
    DriverNotFound,
    SeatNotFound,
}

impl Error for LibError {}

async fn insert_race_results(pool: Pool<Postgres>, config: &Config) -> Result<(), Box<dyn Error>> {
    let mut transaction = pool.begin().await?;

    let result = fileparser::read_race_result(&config.filepath)?;

    let race_id = result.race_id;
    let season = result.season;

    if !race_exists(race_id, season, transaction.as_mut()).await? {
        return Err(Box::new(LibError::RaceDoesntExist));
    }

    for result in result.results {
        let seat_id = result.seat_id.unwrap_or(get_seat(result.driver_id, transaction.as_mut()).await?);

        if !check_seat(seat_id, transaction.as_mut()).await? {
            return Err(Box::new(LibError::SeatNotFound));
        }

        insert_personal_result(&result, race_id, seat_id, season, &mut transaction).await?;
    }

    transaction.commit().await.map_err(Box::new)?;
    Ok(())
}

async fn get_db_connection(config: &Config) -> Result<Pool<Postgres>, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .connect(&config.database_url)
        .await
}

async fn insert_personal_result<'c>(
    personal_result: &PersonRaceResult,
    race_id: i32,
    seat_id: i32,
    season: i32,
    executor: &mut Transaction<'c, Postgres>,
) -> Result<(), sqlx::Error> {
    let result_id = insert_race_result(
        &(*personal_result).into(),
        race_id,
        season,
        executor.as_mut(),
    )
    .await?;

    sqlx::query!("INSERT INTO has_result VALUES ($1, $2)", seat_id, result_id)
        .execute(executor.as_mut())
        .await?;
    Ok(())
}

async fn insert_race_result<'c, T>(
    race_result: &RaceResult,
    race_id: i32,
    season: i32,
    executor: &'c mut T,
) -> Result<i32, sqlx::Error>
where
    &'c mut T: Executor<'c, Database = Postgres>,
{
    let result = sqlx::query!(
        "INSERT INTO result 
        (position, pole, leading_lap, fastest_lap, qualy_result, season, race_id, bot_result) 
            VALUES 
            ($1, $2,$3,$4,$5,$6, $7, $8)
            RETURNING result_id",
        race_result.position,
        race_result.pole,
        race_result.leading_lap,
        race_result.fastest_lap,
        race_result.qualy_result,
        season,
        race_id,
        race_result.bot_result
    )
    .fetch_one(executor)
    .await?;

    Ok(result.result_id)
}

async fn race_exists<'c, T>(
    race: i32,
    _season: i32,
    executor: &'c mut T,
) -> Result<bool, Box<dyn Error>>
where
    &'c mut T: Executor<'c, Database = Postgres>,
{
    match sqlx::query("SELECT * FROM races WHERE race_id = $1")
        .bind(race)
        .fetch_one(executor)
        .await
    {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(e) => Err(Box::new(e)),
    }
}

async fn get_seat<'c, T>(driver_id: i32, executor: &'c mut T) -> Result<i32, Box<dyn Error>>
where
    &'c mut T: Executor<'c, Database = Postgres>,
{
    match sqlx::query!(
        "SELECT seat_id FROM drives_in WHERE driver_id = $1",
        driver_id
    )
    .fetch_all(executor)
    .await
    {
        Ok(data) => {
            if data.is_empty() {
                Err(Box::new(LibError::SeatNotFound))
            } else {
                Ok(data
                    .iter()
                    .max_by(|x, y| x.seat_id.cmp(&y.seat_id))
                    .unwrap()
                    .seat_id)
            }
        }
        Err(sqlx::Error::RowNotFound) => Err(Box::new(LibError::SeatNotFound)),
        Err(e) => Err(Box::new(e)),
    }
}

async fn check_seat<'c, T>(seat_id: i32, executor: &'c mut T) -> Result<bool, sqlx::Error>
where
    &'c mut T: Executor<'c, Database = Postgres>,
{
    match sqlx::query!("SELECT seat_id FROM drives_in WHERE seat_id = $1", seat_id)
        .fetch_one(executor)
        .await
    {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(e) => Err(e),
    }
}

async fn insert_points(pool: Pool<Postgres>) {
    let positions = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 101,
        111, 100,
    ];

    let season = 3;

    for position in positions {
        for pole in [true, false] {
            for fastest_lap in [true, false] {
                for leading_lap in [true, false] {
                    let points = calculate_points(position, pole, fastest_lap, leading_lap);

                    sqlx::query!("INSERT INTO points (season, position, pole, leading_lap, fastest_lap, points) VALUES ($1,$2,$3,$4,$5,$6)", season, position, pole, leading_lap, fastest_lap, points).execute(&pool).await.unwrap();
                }
            }
        }
    }
}

fn calculate_points(position: i32, pole: bool, fastest_lap: bool, leading_lap: bool) -> i32 {
    let points = [20, 16, 12, 10, 8, 6, 4, 3, 2, 1];

    let flat_points = points.get(position as usize - 1).unwrap_or(&0);
    let fastest_lap = if fastest_lap { 2 } else { 0 };
    let leading_lap = if leading_lap { 1 } else { 0 };
    let pole_points = if pole { 2 } else { 0 };

    if position != 111 {
        flat_points + fastest_lap + leading_lap + pole_points
    } else {
        pole_points
    }
}
