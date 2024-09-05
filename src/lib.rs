use std::{
    error::Error, ffi::OsString, path::{Path, PathBuf}
};


use sqlx::{Pool, Postgres};

#[derive(Debug)]
pub struct Config{
    pub database_url: String,
    pub mode: ConfigMode,
    pub filepath: String,
}
#[derive(Debug)]
pub enum ConfigMode {
    RaceResult,
}

async fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let pool = get_db_connection(config).await.map_err(|x| Box::new(x))?;

    match config.mode {
        ConfigMode::RaceResult => insert_race_results(pool, config).await,
    }
}
async fn insert_race_results(pool: Pool<Postgres>, config: &Config) -> Result<(), Box<dyn Error>> {


    todo!()
}

async fn get_db_connection(config: &Config) -> Result<Pool<Postgres>, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .connect(&config.database_url)
        .await
}
