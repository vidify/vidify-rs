mod data;

use core::api::{init_api, API};
use core::config;

fn main() {
    let which: API = API::MPRIS;
    let api = init_api(which);
    println!("Player name: {}", api.get_player_name());
    let config = config::init_config();
    println!("Config: {:#?}", config);
}
