mod data;

use core::api::{init_api, API};
use core::config;

fn main() {
    let which: API = API::MPRIS;
    let config = config::init_config().unwrap();
    println!("Config: {:#?}", config);
    let api = init_api(which, &config).unwrap();
    println!("Player name: {}", (*api).get_player_name());
}
