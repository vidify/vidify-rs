use core::api::{init_api, API};
use core::config::Config;

fn main() {
    let which: API = API::MPRIS;
    let api = init_api(which);
    // println!("Player name: {}", api.get_player_name());
    let conf = Config::new().unwrap();
    println!("Config: {:?}", conf);
}
