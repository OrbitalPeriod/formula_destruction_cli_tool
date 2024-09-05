use std::{fs, path::Path};

use formula_destruction_cli_tool::{Config, ConfigMode, run};
mod fileparser;


#[tokio::main]
async fn main() {
    let args = std::env::args();
    let config = parse_config(args);
    
    match run(&config).await{
        Ok(_) => println!("Program executed succefully"),
        Err(e) => eprintln!("Something went wrong: {}", e)
    }
}


fn parse_config(args : std::env::Args) -> Config{
    dotenv::dotenv().expect("Failed to read .env file");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut iter = args.skip(1);
    let mode = iter.next();
    let config_mode = match mode{
        Some(value) => {
            match value.to_lowercase().as_str(){
                "r" => ConfigMode::RaceResult,
                _ => panic!("Invalid race result input")
            }
        },
        None => {
            panic!("No value given");
        }
    };

    let file_path = iter.next();
    let file_path = match file_path{
        Some(path) => {
            let path_obj = Path::new(&path);
            if path_obj.exists(){
                path
            }else{
                panic!("File does not exist")
            }
        },
        None => panic!("No path given")
    };

    Config{
        database_url: db_url,
        mode : config_mode,
        filepath: file_path,
    }
}