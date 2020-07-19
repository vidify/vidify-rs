use std::fs::File;

use core::api::{init_api, API};
use core::config::init_config;
use core::data::{Res, ResKind};
use simplelog::{CombinedLogger, TermLogger, WriteLogger, LevelFilter, Config, TerminalMode};

fn main() {
    // Initializing the config file
    let config = init_config().unwrap();
    println!("Config: {:#?}", config);

    // Initializing the logger
    let file: &str = &Res::new(ResKind::Data(String::from("session.log"))).expect("Couldn't create log file");
    CombinedLogger::init(vec![
        TermLogger::new(
            if config.debug { LevelFilter::Trace } else { LevelFilter::Off },
            Config::default(),
            TerminalMode::Stderr
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::open(file).unwrap(),
        ),
    ]).unwrap();
    log::info!("Initialized the logger");
    log::info!("Config: {:?}", config);

    // Initializing the API
    let api = init_api(
        config.api.clone().unwrap_or(API::SpotifyWeb),
        &config,
    );
    match api {
        Ok(api) => {
            println!("Player data:");
            println!("    Player name: {}", (*api).get_player_name());
            println!("    Artist: {:?}", (*api).get_artist());
            println!("    Title: {:?}", (*api).get_title());
            println!("    Position: {:?}", (*api).get_position());
            println!("    Is playing?: {}", (*api).is_playing());
        },
        Err(err) => eprintln!("Error: {}", err.to_string()),
    }
}
