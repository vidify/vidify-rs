use crate::player::Player;
use crate::api::API;

use config_macro::Conf;
use ini::Ini;
use dirs;
use std::fs::File;

macro_rules! opt {
    ($args: tt, $ini: tt, $conf_name: tt, $arg_names: tt, $type: ty, $default: tt) => ({
        {
            // First checking the arguments
            let mut ret: Option<$type> = None;
            for name in $arg_names.iter() {
                match $args.opt_value_from_str::<&str, $type>(name) {
                    // May fail in some cases.
                    Ok(val) => match val {
                        // May not exist.
                        Some(val) => {
                            ret = Some(val);
                            break;
                        },
                        None => continue
                    },
                    Err(_) => continue
                }
            }

            match ret {
                Some(val) => val,
                None => {
                    // Then the config file, falling back to the default value
                    $ini.get_from_or(Some("Defaults"), $conf_name, $default)
                    .parse::<$type>()
                    .expect(concat!("Could not parse the value of '",
                                    $conf_name, "' in the config file."))
                }
            }
        }
    });
}

#[derive(Debug, Conf)]
pub struct Config {
    #[conf(long = "--debug", help = "debug mode")]
    debug: bool,
    // config_file: String,
    // #[conf(short = "-n", long = "--no-lyrics", inverted_arg = true,
           // help = "no lyrics")]
    // lyrics: bool,
    // fullscreen: bool,
    // dark_mode: bool,
    // stay_on_top: bool,
    // width: u32,
    // height: u32,
    // api: Option<API>,
    // player: Option<Player>,
    // audiosync: bool,
    // audiosync_calibration: i32,
    // vlc_args: String,
    // mpv_flags: String,
    // #[conf(long = "--client-id", help = "no lyrics")]
    // client_id: String,
    // client_secret: String,
    // redirect_uri: String,
    // refresh_token: String,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();

        // Reading the file from the user's config directory.
        let mut config_file = dirs::config_dir()
            .expect("Could not find user config directory");
        config_file.push("vidify");
        config_file.push("config.ini");
        let config_file_str = config_file.to_str()
            .expect("Invalid UTF-8 characters found in the config path");

        // Checking that the config file exists, and creating it otherwise.
        if !config_file.as_path().exists() {
            File::create(&config_file).expect("Could not create config file");
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

        // Should be automatically generated
        if args.contains(["-h", "--help"]) {
            println!("help goes here");
        }

        if args.contains(["-v", "--version"]) {
            println!("version goes here");
        }

        Ok(Config {
            debug: opt!(args, ini, "debug", ["--debug", "-d"], bool, "true"),
            // debug: args
                // .opt_value_from_str("--debug")?
                // .unwrap_or(
                    // ini.get_from_or(Some("Defaults"), "debug", "true")
                    // .parse::<bool>()
                    // .expect("Could not parse the value of 'debug' in the \
                            // config file.")
                // )
        })
    }
}
