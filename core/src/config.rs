use crate::api::API;
use crate::player::Player;

use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use dirs;
use ini::Ini;
use std::fs::File;
use std::path::PathBuf;

macro_rules! opt {
    ($args: tt, $ini: tt, $type: ty, $name: expr, $section: expr) => {
        // The arguments have more priority than config file. Once one
        // of them is obtained, it's parsed.
        $args
            .value_of($name)
            .or_else(|| $ini.get_from(Some($section), $name))
            .and_then(|x| {
                let err_msg = concat!(
                    "Could not parse the value of '",
                    $name,
                    "' in the config file."
                );
                Some(x.parse::<$type>().expect(err_msg))
            })
    };
}

macro_rules! arg {
    ($name: expr, $help: expr) => {
        Arg::with_name($name)
            .long($name.replace("_", "-").as_str())
            .help($help)
    };
    ($name: expr, $short: expr, $help: expr) => {
        Arg::with_name($name)
            .long($name.replace("_", "-").as_str())
            .short($short)
            .help($help)
    };
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
    mpv_flags: String,
    client_id: Option<String>,
    client_secret: Option<String>,
    redirect_uri: String,
    refresh_token: Option<String>,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let a = Config::parse_args();
        let f = Config::parse_config_file(a.value_of("config_file"));
        Ok(Config {
            debug: opt!(a, f, bool, "debug", "Defaults").unwrap_or(true),
            lyrics: opt!(a, f, bool, "no_lyrics", "Defaults")
                .and_then(|x| Some(!x))
                .unwrap_or(true),
            fullscreen: opt!(a, f, bool, "fullscreen", "Defaults")
                .unwrap_or(false),
            stay_on_top: opt!(a, f, bool, "stay_on_top", "Defaults")
                .unwrap_or(false),
            api: opt!(a, f, API, "api", "Defaults"),
            player: opt!(a, f, Player, "player", "Defaults"),
            audiosync: opt!(a, f, bool, "audiosync", "Defaults")
                .unwrap_or(true),
            audiosync_calibration: opt!(
                a,
                f,
                i32,
                "audiosync_calibration",
                "Defaults"
            )
            .unwrap_or(0),
            mpv_flags: opt!(a, f, String, "mpv_flags", "Defaults")
                .unwrap_or(String::from("")),
            client_id: opt!(a, f, String, "client_id", "SpotifyWeb"),
            client_secret: opt!(a, f, String, "client_secret", "SpotifyWeb"),
            redirect_uri: opt!(a, f, String, "redirect_uri", "SpotifyWeb")
                .unwrap_or(String::from("")),
            refresh_token: opt!(a, f, String, "refresh_token", "SpotifyWeb"),
        })
    }

    fn parse_config_file(path_str: Option<&str>) -> Ini {
        // The path is first obtained as a `String` for output and
        let path = match path_str {
            Some(path) => PathBuf::from(path),
            None => {
                let mut path = dirs::config_dir()
                    .expect("Could not find user config directory");
                path.extend(["vidify", "config.ini"].iter());
                path
            }
        };

        // Checking that the config file exists, and creating it otherwise.
        if !path.exists() {
            File::create(&path).expect("Could not create config file");
            println!(
                "Created config file at {}",
                path.to_str().expect(
                    "Invalid UTF-8 characters found in the config path"
                )
            );
        }
        Ini::load_from_file(path).unwrap()
    }

    fn parse_args<'a>() -> ArgMatches<'a> {
        // Basic information about the program.
        let app = App::new("vidify")
            .version(crate_version!())
            .author(crate_authors!());

        // All the available options as arguments.
        app.args(&[
            arg!("debug", "d", "display debug messages."),
            arg!("config_file", "the config file path."),
            arg!("no_lyrics", "n", "do not print lyrics."),
            arg!("fullscreen", "f", "open the app in fullscreen mode."),
            arg!("stay_on_top", "the window will stay on top of all apps."),
            arg!(
                "api",
                "a",
                "the source music player used. Read the installation guide \
                for a list with the available APIs."
            ),
            arg!(
                "player",
                "p",
                "the output video player. Read the installation guide for a \
                list with the available players."
            ),
            arg!(
                "audiosync",
                "enable automatic audio synchronization. Read the \
                installation guide for more information. Note: this feature \
                is still in development."
            ),
            arg!(
                "audiosync_callibration",
                "manual tweaking value for audiosync in milliseconds."
            ),
            arg!(
                "mpv_flags",
                "custom boolean flags used when opening mpv, with dashes and \
                separated by spaces."
            ),
            arg!(
                "client_id",
                "the client ID for the Spotify Web API. Check the guide to \
                learn how to obtain yours."
            ),
            arg!(
                "client_secret",
                "the client secret for the Spotify Web API. Check the \
                install guide to learn how to obtain yours."
            ),
        ])
        .get_matches()
    }
}
