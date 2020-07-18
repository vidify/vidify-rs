mod data;

use core::api::{init_api, API};
use core::config;

fn main() {
    let config = config::init_config().unwrap();
    println!("Config: {:#?}", config);
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
