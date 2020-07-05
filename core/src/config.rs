use crate::player::Player;
use crate::api::API;

use ini::Ini;
use dirs;
use std::fs::File;

macro_rules! opt {
    ($ini: tt, $name: tt, $section: tt, $default: tt) => ({
        {
            // Then the config file, falling back to the default value
            match $ini.get_from(Some($section), $name) {
                Some(val) => {
                    val.parse()
                       .expect(concat!("Could not parse the value of '",
                                       $name, "' in the config file."))
                },
                None => $default
            }
        }
    });
}

macro_rules! opt_nodef {
    ($ini: tt, $name: tt, $section: tt) => ({
        {
            // Then the config file, falling back to the default value
            $ini.get_from(Some($section), $name).and_then(|val| {
                Some(val.parse()
                        .expect(concat!("Could not parse the value of '",
                                        $name, "' in the config file.")))
            })
        }
    });
}


// #[derive(Debug, Conf)]
#[derive(Debug)]
pub struct Config {
    debug: bool,
    lyrics: bool,
    fullscreen: bool,
    // dark_mode: bool,
    stay_on_top: bool,
    // width: u32,
    // height: u32,
    api: Option<API>,
    player: Option<Player>,
    audiosync: bool,
    audiosync_calibration: i32,
    vlc_args: Option<String>,
    mpv_flags: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    redirect_uri: String,
    refresh_token: Option<String>,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        // Reading the file from the user's config directory.
        let mut file = dirs::config_dir()
            .expect("Could not find user config directory");
        file.push("vidify");
        file.push("config.ini");
        let file_str = file.to_str()
            .expect("Invalid UTF-8 characters found in the config path");

        // Checking that the config file exists, and creating it otherwise.
        if !file.as_path().exists() {
            File::create(&file).expect("Could not create config file");
            println!("Created config file at {}", file_str);
        }
        let ini = Ini::load_from_file(file_str).unwrap();

        Ok(Config {
            debug: opt!(ini, "debug", "Defaults", true),
            lyrics: opt!(ini, "lyrics", "Defaults", true),
            fullscreen: opt!(ini, "fullscreen", "Defaults", false),
            stay_on_top: opt!(ini, "stay_on_top", "Defaults", false),
            api: opt_nodef!(ini, "api", "Defaults"),
            player: opt_nodef!(ini, "player", "Defaults"),
            audiosync: opt!(ini, "audiosync", "Defaults", true),
            audiosync_calibration: opt!(ini, "audiosync_calibration", "Defaults", 0),
            vlc_args: opt_nodef!(ini, "vlc_args", "Defaults"),
            mpv_flags: opt_nodef!(ini, "mpv_flags", "Defaults"),
            client_id: opt_nodef!(ini, "client_id", "SpotifyWeb"),
            client_secret: opt_nodef!(ini, "client_secret", "SpotifyWeb"),
            redirect_uri: opt!(ini, "redirect_uri", "SpotifyWeb", (String::from("http://localhost:8888/callback/"))),
            refresh_token: opt_nodef!(ini, "refresh_token", "SpotifyWeb"),
        })
    }
}
