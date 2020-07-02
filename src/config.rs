use crate::player::Player;
use crate::api::API;

use ini::Ini;
use dirs;
use std::fs::File;

pub struct Config {
    debug: bool,
    config_file: String,
    lyrics: bool,
    fullscreen: bool,
    dark_mode: bool,
    stay_on_top: bool,
    width: u32,
    height: u32,
    api: API,
    player: Player,
    audiosync: bool,
    audiosync_calibration: i32,
    vlc_args: String,
    mpv_flags: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    refresh_token: String,
}

impl Config {
    const DEFAULT_SECTION: &'static str = "Defaults";

    pub fn new() -> Config {
        // Reading the file from the user's config directory.
        let mut config_file = dirs::config_dir()
            .expect("Could not find user config directory");
        config_file.push("vidify");
        config_file.push("config.ini");
        let config_file_str = config_file.to_str()
            .expect("Invalid UTF-8 characters found in the config path");

        // Checking that the config file exists, and creating it otherwise.
        if !config_file.as_path().exists() {
            File::create(config_file).expect("Could not create config file");
            println!("Created config file at {}", config_file_str);
        }

        // TODO: reading a new file is useless
        // Serializing the config file values. After the previous check,
        // it must exist.
        let ini = Ini::load_from_file(config_file_str).unwrap();
        // TODO: remove, this is for debugging
        for (sec, prop) in ini.iter() {
            println!("Section: {:?}", sec);
            for (k, v) in prop.iter() {
                println!("{}:{}", k, v);
            }
        }

        Config {
            debug: ini.get_from(super::DEFAULT_SECTION, "debug"),
            // ...
        }
    }
}
