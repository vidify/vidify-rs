mod api;
mod base;
mod config;
mod player;

use crate::api::{init_api, API};

fn main() {
    let which: API = API::MPRIS;
    let api = init_api(which);
    println!("Player name: {}", api.get_player_name())
}
